#!/usr/bin/env bash

mkdir -p src/main/jniLibs/armeabi-v7a
mkdir -p src/main/jniLibs/arm64
mkdir -p src/main/jniLibs/x86

download_libgnustl(){
    # rm -rf indy-android-dependencies
    # git clone https://github.com/faisal00813/indy-android-dependencies
    pushd indy-android-dependencies/prebuilt/libgnustl
    unzip libgnustl_arm64.zip
    cp libgnustl_arm64/libgnustl_shared.so ../../../../wrappers/android/src/main/jniLibs/arm64/
    rm libgnustl_arm64/libgnustl_shared.so

    unzip libgnustl_armv7.zip
    cp libgnustl_armv7/libgnustl_shared.so ../../../../wrappers/android/src/main/jniLibs/armeabi-v7a/
    rm libgnustl_armv7/libgnustl_shared.so

    unzip libgnustl_x86.zip
    cp libgnustl_x86/libgnustl_shared.so ../../../../wrappers/android/src/main/jniLibs/libgnustl_x86/
    rm libgnustl_x86/libgnustl_shared.so
    popd
}

copy_native_binaries(){
    cp libindy_arm/lib/libindy.so ../wrappers/android/src/main/jniLibs/armeabi-v7a/
    cp libindy_arm64/lib/libindy.so ../wrappers/android/src/main/jniLibs/arm64/
    cp libindy_x86/lib/libindy.so ../wrappers/android/src/main/jniLibs/x86/
}

pushd ../../libindy
    sh build-libindy-android.sh
    copy_native_binaries
    download_libgnustl
popd


cp -r ../java/src/main/java src/main/
cp -r ../java/src/main/resources src/main/


javaDir=src/main/java
javaFiles=`find $javaDir -type f -name "*.java"`

for f in $javaFiles; do sed -i 's/java.util.concurrent.CompletableFuture/java9.util.concurrent.CompletableFuture/g' "$f"; done

chmod +x gradlew
./gradlew clean assemble

mv build/outputs/aar/*-debug.aar build/outputs/aar/libindy-debug.aar
rm ../../samples/android/WrapperTest/libindy-debug/libindy-debug.aar
cp build/outputs/aar/libindy-debug.aar ../../samples/android/WrapperTest/libindy-debug/