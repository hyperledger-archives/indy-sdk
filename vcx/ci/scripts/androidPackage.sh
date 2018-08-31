#!/bin/bash

set -e
ANDROID_JNI_LIB=vcx/wrappers/java/vcx/src/main/jniLibs

mkdir -p ${ANDROID_JNI_LIB}/arm
mkdir -p ${ANDROID_JNI_LIB}/x86
mkdir -p ${ANDROID_JNI_LIB}/arm64
cp -v runtime_android_build/libvcx_arm/libvcx.so ${ANDROID_JNI_LIB}/arm/libvcx.so
cp -v runtime_android_build/libvcx_x86/libvcx.so ${ANDROID_JNI_LIB}/x86
cp -v runtime_android_build/libvcx_arm64/libvcx.so ${ANDROID_JNI_LIB}/arm64

pushd vcx/wrappers/java/vcx
    if [ -e local.properties ]; then
       rm local.properties
    fi
cat <<EOT >> local.properties
ndk.dir=/home/vcx/android-sdk-linux/ndk-bundle
sdk.dir=/home/vcx/android-sdk-linux
EOT

    ./gradlew --no-daemon test
    ./gradlew --no-daemon clean assemble
popd
