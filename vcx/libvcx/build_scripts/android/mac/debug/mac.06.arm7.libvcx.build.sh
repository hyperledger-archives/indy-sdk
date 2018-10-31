#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

source ./mac.05.libvcx.env.sh
cd ../../../..

export ORIGINAL_PATH=$PATH
#export ORIGINAL_PKG_CONFIG_PATH=$PKG_CONFIG_PATH

# Commenting because we don't want to compile cargo everytime
# cargo clean
cargo install

export OPENSSL_DIR_DARWIN=$OPENSSL_DIR

# KS: Commenting it out because we want to debug only on armv7 based device/simulator

# export PATH=$WORK_DIR/NDK/arm/bin:$ORIGINAL_PATH
# export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-armeabi
# export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_arm/lib
# export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_arm/lib
# export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/arm-linux-androideabi/release
# cargo build --target arm-linux-androideabi --release --verbose

export PATH=$WORK_DIR/NDK/arm/bin:$ORIGINAL_PATH
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-armeabi-v7a
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_armv7/lib
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_armv7/lib
export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/armv7-linux-androideabi/release
# export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/armv7-linux-androideabi/debug
cargo build --target armv7-linux-androideabi
# cargo build --target armv7-linux-androideabi --release

# KS: Commenting it out because we want to debug only on armv7 based device/simulator

# export PATH=$WORK_DIR/NDK/arm64/bin:$ORIGINAL_PATH
# export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-arm64-v8a
# export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_arm64/lib
# export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_arm64/lib
# export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/aarch64-linux-android/release
# cargo build --target aarch64-linux-android --release --verbose

# export PATH=$WORK_DIR/NDK/x86/bin:$ORIGINAL_PATH
# export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-x86
# export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_x86/lib
# export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_x86/lib
# export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/i686-linux-android/release
# cargo build --target i686-linux-android --release --verbose

# export PATH=$WORK_DIR/NDK/x86_64/bin:$ORIGINAL_PATH
# export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-x86_64
# export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_x86_64/lib
# export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_x86_64/lib
# export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/x86_64-linux-android/release
# cargo build --target x86_64-linux-android --release --verbose

# This builds the library for code that runs in OSX
ln -sf $WORK_DIR/vcx-indy-sdk/libindy/target/x86_64-apple-darwin/release/libindy.dylib /usr/local/lib/libindy.dylib
export PATH=$ORIGINAL_PATH
export OPENSSL_DIR=$OPENSSL_DIR_DARWIN
unset ANDROID_SODIUM_LIB
unset ANDROID_ZMQ_LIB
unset LIBINDY_DIR
# cargo build --target x86_64-apple-darwin --release --verbose

#cargo test

#export PKG_CONFIG_PATH=$ORIGINAL_PKG_CONFIG_PATH








# To build for macos
#cargo build
#export LIBINDY_DIR=/usr/local/lib
#export RUST_BACKTRACE=1
# To build for iOS
#LIBINDY_DIR=/usr/local/lib RUST_BACKTRACE=1 cargo lipo --release

#cargo lipo --release --verbose --targets="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"

#LIBINDY_DIR=/usr/local/lib RUST_BACKTRACE=1 cargo lipo
#LIBINDY_DIR=/usr/local/lib cargo test
