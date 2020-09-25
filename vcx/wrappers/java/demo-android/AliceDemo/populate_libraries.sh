#!/usr/bin/env bash

workdir=output
jnadir=${workdir}/jna
jniLibs=${workdir}/jniLibs
libindy_version=1.15.0
libvcx_version=0.8.0

mkdir -p ${jniLibs}/armeabi-v7a
mkdir -p ${jniLibs}/arm64-v8a
mkdir -p ${jniLibs}/x86
mkdir -p ${jnadir}

download_prebuilt_libindy(){
    pushd ${workdir}
    wget -O libindy_android_armv7_${libindy_version}.zip "https://repo.sovrin.org/android/libindy/stable/${libindy_version}/libindy_android_armv7_${libindy_version}.zip"
    unzip libindy_android_armv7_${libindy_version}.zip

    wget -O libindy_android_arm64_${libindy_version}.zip "https://repo.sovrin.org/android/libindy/stable/${libindy_version}/libindy_android_arm64_${libindy_version}.zip"
    unzip libindy_android_arm64_${libindy_version}.zip

    wget -O libindy_android_x86_${libindy_version}.zip "https://repo.sovrin.org/android/libindy/stable/${libindy_version}/libindy_android_x86_${libindy_version}.zip"
    unzip libindy_android_x86_${libindy_version}.zip
    popd 
}

download_prebuilt_libvcx(){
    pushd ${workdir}
    wget -O libvcx_android_armv7_${libvcx_version}.zip "https://repo.sovrin.org/android/libvcx/stable/${libvcx_version}/libvcx_android_armv7_${libvcx_version}.zip"
    unzip libvcx_android_armv7_${libvcx_version}.zip

    wget -O libvcx_android_arm64_${libvcx_version}.zip "https://repo.sovrin.org/android/libvcx/stable/${libvcx_version}/libvcx_android_arm64_${libvcx_version}.zip"
    unzip libvcx_android_arm64_${libvcx_version}.zip

    wget -O libvcx_android_x86_${libvcx_version}.zip "https://repo.sovrin.org/android/libvcx/stable/${libvcx_version}/libvcx_android_x86_${libvcx_version}.zip"
    unzip libvcx_android_x86_${libvcx_version}.zip
    popd
}

download_ndk(){
    pushd ${workdir}
    if [ "$(uname)" == "Darwin" ]; then
        echo "Downloading NDK for macOS"
        wget -O ndk_r21b.zip "https://dl.google.com/android/repository/android-ndk-r21b-darwin-x86_64.zip"
    elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
        echo "Downloading NDK for Linux"
        wget -O ndk_r21b.zip "https://dl.google.com/android/repository/android-ndk-r21b-linux-x86_64.zip"
    fi
    unzip ndk_r21b.zip
    popd 
}

download_jna(){
    pushd ${jnadir}
    wget -O jna-android-armv7.jar "https://github.com/java-native-access/jna/raw/4.5.2/lib/native/android-armv7.jar"
    wget -O jna-android-arm64.jar "https://github.com/java-native-access/jna/raw/4.5.2/lib/native/android-aarch64.jar"
    wget -O jna-android-x86.jar "https://github.com/java-native-access/jna/raw/4.5.2/lib/native/android-x86.jar"
    popd
}

copy_native_libraries(){
    pushd ${workdir}
    cp libindy_armv7/lib/libindy.so jniLibs/armeabi-v7a/
    cp libindy_arm64/lib/libindy.so jniLibs/arm64-v8a/
    cp libindy_x86/lib/libindy.so jniLibs/x86/

    cp libvcx_armv7/lib/libvcx.so jniLibs/armeabi-v7a/
    cp libvcx_arm64/lib/libvcx.so jniLibs/arm64-v8a/
    cp libvcx_x86/lib/libvcx.so jniLibs/x86/

    cp android-ndk-r21b/sources/cxx-stl/llvm-libc++/libs/armeabi-v7a/libc++_shared.so jniLibs/armeabi-v7a/
    cp android-ndk-r21b/sources/cxx-stl/llvm-libc++/libs/arm64-v8a/libc++_shared.so jniLibs/arm64-v8a/
    cp android-ndk-r21b/sources/cxx-stl/llvm-libc++/libs/x86/libc++_shared.so jniLibs/x86/

    unzip jna/jna-android-armv7.jar libjnidispatch.so -d jniLibs/armeabi-v7a/
    unzip jna/jna-android-arm64.jar libjnidispatch.so -d jniLibs/arm64-v8a/
    unzip jna/jna-android-x86.jar libjnidispatch.so -d jniLibs/x86/

    cp -r jniLibs ../app/src/main
    popd
}

cleanup(){
    rm -rf output
}

download_prebuilt_libindy
download_prebuilt_libvcx
download_ndk
download_jna
copy_native_libraries
cleanup
