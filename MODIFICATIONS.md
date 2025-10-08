
# Модификации для stealth версии

Чтобы сделать stealth версию, нужно:

1. **Убрать UI** - изменить AndroidManifest.xml:
   - Удалить LAUNCHER intent-filter
   - Сделать тему NoDisplay

2. **Авто-запуск** - добавить BroadcastReceiver для:
   - BOOT_COMPLETED
   - ACTION_POWER_CONNECTED

3. **Авто-разрешения** - программно запрашивать:
   - overlay permission
   - accessibility service
   - screen capture

4. **Отправка в Telegram** - добавить код для отправки:
   - ID устройства
   - Пароля
   - На твой Telegram через бота

Для этого нужны изменения в:
- android/app/src/main/AndroidManifest.xml
- android/app/src/main/java/.../MainActivity.java
- lib/main.dart
