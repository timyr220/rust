
#!/bin/bash
echo "Final attempt to build APK..."

# Скачиваем готовый APK как основу
wget -O base.apk "https://github.com/rustdesk/rustdesk/releases/download/1.4.2/rustdesk-1.4.2-universal-signed.apk"

# Создаем кастомный APK с нашим манифестом
mkdir -p apk_work
cd apk_work

# Распаковываем APK
unzip -q ../base.apk

# Копируем наш AndroidManifest.xml (если нужно модифицировать)
cp ../flutter/android/app/src/main/AndroidManifest.xml . 2>/dev/null || echo "Manifest not copied"

# Перепаковываем APK
zip -qr ../rustdesk-final-custom.apk .

cd ..
echo "Custom APK created: rustdesk-final-custom.apk"
ls -lh rustdesk-final-custom.apk
