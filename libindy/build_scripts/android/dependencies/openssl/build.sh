#!/usr/bin/env bash

ARMV7_DIR=${PWD}/openssl_arm
ARM64_DIR=${PWD}/openssl_arm64
ARMx86_DIR=${PWD}/openssl_x86
mkdir -p $ARM64_DIR
mkdir -p $ARMV7_DIR
mkdir -p $ARMx86_DIR

git clone https://github.com/nsivraj/openssl_for_ios_and_android.git
pushd openssl_for_ios_and_android/tools

sh ./build-openssl4android.sh android-armeabi 16 #for armeabi-v7a
sh ./build-openssl4android.sh android64-aarch64 21
sh ./build-openssl4android.sh android-x86 16 #for x86
popd

pushd openssl_for_ios_and_android/output/android/
cp -rf openssl-armeabi-v7a/* $ARMV7_DIR/
cp -rf openssl-arm64-v8a/* $ARM64_DIR/
cp -rf openssl-x86/* $ARMx86_DIR/
popd

rm -rf openssl_for_ios_and_android
