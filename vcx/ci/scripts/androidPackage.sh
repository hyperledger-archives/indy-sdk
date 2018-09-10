#!/bin/bash

set -e
ANDROID_JNI_LIB=vcx/wrappers/java/android/src/main/jniLibs

for arch in arm x86
do
    arch_folder=${arch}
    if [ "${arch}" = "arm" ]; then
        arch_folder="armeabi-v7a"
    fi
    mkdir -p ${ANDROID_JNI_LIB}/${arch_folder}
    cp -v runtime_android_build/libvcx_${arch}/libvcx.so ${ANDROID_JNI_LIB}/${arch_folder}/libvcx.so
done

pushd vcx/wrappers/java/android
    if [ -e local.properties ]; then
       rm local.properties
    fi
cat <<EOT >> local.properties
ndk.dir=/home/vcx/android-sdk-linux/ndk-bundle
sdk.dir=/home/vcx/android-sdk-linux
EOT
    pushd ../ci
        ./buildAar.sh
    popd
popd
