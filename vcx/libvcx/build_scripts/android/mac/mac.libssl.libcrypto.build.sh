#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

if [ -d $WORK_DIR/openssl_for_ios_and_android ]; then
    #rm -rf $WORK_DIR/openssl_for_ios_and_android
    cd $WORK_DIR/openssl_for_ios_and_android
    git checkout .
    git clean -f
    git clean -fd
    git pull
else
    git clone https://github.com/nsivraj/openssl_for_ios_and_android.git $WORK_DIR/openssl_for_ios_and_android
    cd $WORK_DIR/openssl_for_ios_and_android
fi

cd tools
sh ./build-openssl4android.sh android 16 # for armeabi
sh ./build-openssl4android.sh android-armeabi 16 #for armeabi-v7a
sh ./build-openssl4android.sh android64-aarch64 21 #for arm64_v8a
sh ./build-openssl4android.sh android-x86 16 #for x86
sh ./build-openssl4android.sh android64 21 #for x86_64
