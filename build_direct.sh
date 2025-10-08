
#!/bin/bash
echo "Building APK directly through Android SDK..."

cd flutter/android

# Создаем локальный properties файл если нужно
if [ ! -f "local.properties" ]; then
    echo "Creating local.properties..."
    echo "sdk.dir=$ANDROID_HOME" > local.properties
fi

# Пробуем собрать напрямую через Gradle
if [ -f "gradlew" ]; then
    chmod +x gradlew
    ./gradlew clean
    ./gradlew assembleRelease
else
    # Если gradlew нет, используем системный Gradle
    gradle clean
    gradle assembleRelease
fi

# Проверяем результат
if [ -f "app/build/outputs/apk/release/app-release.apk" ]; then
    echo "APK built successfully!"
    cp app/build/outputs/apk/release/app-release.apk ../../rustdesk-direct.apk
    ls -lh ../../rustdesk-direct.apk
else
    echo "Direct build failed"
    find . -name "*.apk" -type f
fi
