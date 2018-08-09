#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")
SHA_HASH_DIR=$START_DIR/../..
SHA_HASH_DIR=$(abspath "$SHA_HASH_DIR")
LIBSOVTOKEN_ANDROID_BUILD_URL="https://repo.corp.evernym.com/filely/android/libsovtoken_0.8.1-201807262112-cbb1520_all.zip"

source ./mac.02.libindy.env.sh
if [ -d $WORK_DIR/vcx-indy-sdk ]; then
    #rm -rf $WORK_DIR/vcx-indy-sdk
    cd $WORK_DIR/vcx-indy-sdk
else
    git clone https://github.com/hyperledger/indy-sdk.git $WORK_DIR/vcx-indy-sdk
    cd $WORK_DIR/vcx-indy-sdk
fi
git checkout .
git checkout master
git clean -f
git clean -fd
git pull
git checkout `cat $SHA_HASH_DIR/libindy.commit.sha1.hash.txt`
#cd $WORK_DIR/vcx-indy-sdk
#git checkout tags/v1.3.0
cd $WORK_DIR/vcx-indy-sdk/libindy

export LIBZMQ_LIB_DIR=/usr/local/lib
export LIBZMQ_INCLUDE_DIR=/usr/local/include
sed -i .bak 's/LIBZMQ_LIB_DIR/ANDROID_ZMQ_LIB/' build.rs
###################################################################################################

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_arm ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_arm.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_arm ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_arm.zip
fi

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_armv7 ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_armv7.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_armv7 ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_armv7.zip
fi

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_arm64 ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_arm64.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_arm64 ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_arm64.zip
fi

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_x86 ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_x86.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_x86 ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_x86.zip
fi

if [ ! -d $WORK_DIR/libzmq-android/libsodium/libsodium_x86_64 ]; then
    cd $WORK_DIR/libzmq-android/libsodium
    unzip libsodium_x86_64.zip
fi
if [ ! -d $WORK_DIR/libzmq-android/zmq/libzmq_x86_64 ]; then
    cd $WORK_DIR/libzmq-android/zmq
    unzip libzmq_x86_64.zip
fi

cd $WORK_DIR/vcx-indy-sdk/libindy
export ORIGINAL_PATH=$PATH
#export ORIGINAL_PKG_CONFIG_PATH=$PKG_CONFIG_PATH

cargo clean

export OPENSSL_DIR_DARWIN=$OPENSSL_DIR

#########################################################################################################################
# Now build libindy
#########################################################################################################################
# export PATH=$WORK_DIR/NDK/arm/bin:$ORIGINAL_PATH; echo "PATH: $PATH"
# export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-armeabi; echo "OPENSSL_DIR: $OPENSSL_DIR"
# export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_arm/lib; echo "ANDROID_SODIUM_LIB: $ANDROID_SODIUM_LIB"
# export SODIUM_LIB_DIR=$ANDROID_SODIUM_LIB; echo "SODIUM_LIB_DIR: $SODIUM_LIB_DIR"
# export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_arm/lib; echo "ANDROID_ZMQ_LIB: $ANDROID_ZMQ_LIB"
# #export LIBZMQ_LIB_DIR=$ANDROID_ZMQ_LIB; echo "LIBZMQ_LIB_DIR: $LIBZMQ_LIB_DIR"
# #export LIBZMQ_INCLUDE_DIR=$WORK_DIR/libzmq-android/zmq/libzmq_arm/include; echo "LIBZMQ_INCLUDE_DIR: $LIBZMQ_INCLUDE_DIR"
# export ANDROID_SQLITE_LIB=$WORK_DIR/libsqlite3-android/sqlite3-android/obj/local/armeabi-v7a; echo "ANDROID_SQLITE_LIB: $ANDROID_SQLITE_LIB"
# sed -i .bak 's/\"\"\.as_ptr() as \*const i8/\"\"\.as_ptr() as \*const u8/' src/services/wallet/storage/plugged/mod.rs
# cargo build --target arm-linux-androideabi --release --verbose
# echo "-----------------------------------------------------------------------------------------------"

export PATH=$WORK_DIR/NDK/arm/bin:$ORIGINAL_PATH; echo "PATH: $PATH"
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-armeabi-v7a; echo "OPENSSL_DIR: $OPENSSL_DIR"
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_armv7/lib; echo "ANDROID_SODIUM_LIB: $ANDROID_SODIUM_LIB"
export SODIUM_LIB_DIR=$ANDROID_SODIUM_LIB; echo "SODIUM_LIB_DIR: $SODIUM_LIB_DIR"
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_armv7/lib; echo "ANDROID_ZMQ_LIB: $ANDROID_ZMQ_LIB"
#export LIBZMQ_LIB_DIR=$ANDROID_ZMQ_LIB; echo "LIBZMQ_LIB_DIR: $LIBZMQ_LIB_DIR"
#export LIBZMQ_INCLUDE_DIR=$WORK_DIR/libzmq-android/zmq/libzmq_armv7/include; echo "LIBZMQ_INCLUDE_DIR: $LIBZMQ_INCLUDE_DIR"
export ANDROID_SQLITE_LIB=$WORK_DIR/libsqlite3-android/sqlite3-android/obj/local/armeabi-v7a; echo "ANDROID_SQLITE_LIB: $ANDROID_SQLITE_LIB"
sed -i .bak 's/\"\"\.as_ptr() as \*const i8/\"\"\.as_ptr() as \*const u8/' src/services/wallet/storage/plugged/mod.rs
cargo build --target armv7-linux-androideabi --release
echo "-----------------------------------------------------------------------------------------------"

# export PATH=$WORK_DIR/NDK/arm64/bin:$ORIGINAL_PATH; echo "PATH: $PATH"
# export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-arm64-v8a; echo "OPENSSL_DIR: $OPENSSL_DIR"
# export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_arm64/lib; echo "ANDROID_SODIUM_LIB: $ANDROID_SODIUM_LIB"
# export SODIUM_LIB_DIR=$ANDROID_SODIUM_LIB; echo "SODIUM_LIB_DIR: $SODIUM_LIB_DIR"
# export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_arm64/lib; echo "ANDROID_ZMQ_LIB: $ANDROID_ZMQ_LIB"
# #export LIBZMQ_LIB_DIR=$ANDROID_ZMQ_LIB; echo "LIBZMQ_LIB_DIR: $LIBZMQ_LIB_DIR"
# #export LIBZMQ_INCLUDE_DIR=$WORK_DIR/libzmq-android/zmq/libzmq_arm64/include; echo "LIBZMQ_INCLUDE_DIR: $LIBZMQ_INCLUDE_DIR"
# export ANDROID_SQLITE_LIB=$WORK_DIR/libsqlite3-android/sqlite3-android/obj/local/arm64-v8a; echo "ANDROID_SQLITE_LIB: $ANDROID_SQLITE_LIB"
# sed -i .bak 's/\"\"\.as_ptr() as \*const i8/\"\"\.as_ptr() as \*const u8/' src/services/wallet/storage/plugged/mod.rs
# cargo build --target aarch64-linux-android --release --verbose
# echo "-----------------------------------------------------------------------------------------------"

export PATH=$WORK_DIR/NDK/x86/bin:$ORIGINAL_PATH; echo "PATH: $PATH"
export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-x86; echo "OPENSSL_DIR: $OPENSSL_DIR"
export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_x86/lib; echo "ANDROID_SODIUM_LIB: $ANDROID_SODIUM_LIB"
export SODIUM_LIB_DIR=$ANDROID_SODIUM_LIB; echo "SODIUM_LIB_DIR: $SODIUM_LIB_DIR"
export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_x86/lib; echo "ANDROID_ZMQ_LIB: $ANDROID_ZMQ_LIB"
#export LIBZMQ_LIB_DIR=$ANDROID_ZMQ_LIB; echo "LIBZMQ_LIB_DIR: $LIBZMQ_LIB_DIR"
#export LIBZMQ_INCLUDE_DIR=$WORK_DIR/libzmq-android/zmq/libzmq_x86/include; echo "LIBZMQ_INCLUDE_DIR: $LIBZMQ_INCLUDE_DIR"
export ANDROID_SQLITE_LIB=$WORK_DIR/libsqlite3-android/sqlite3-android/obj/local/x86; echo "ANDROID_SQLITE_LIB: $ANDROID_SQLITE_LIB"
sed -i .bak 's/\"\"\.as_ptr() as \*const u8/\"\"\.as_ptr() as \*const i8/' src/services/wallet/storage/plugged/mod.rs
cargo build --target i686-linux-android --release
echo "-----------------------------------------------------------------------------------------------"

# export PATH=$WORK_DIR/NDK/x86_64/bin:$ORIGINAL_PATH; echo "PATH: $PATH"
# export OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android/output/android/openssl-x86_64; echo "OPENSSL_DIR: $OPENSSL_DIR"
# export ANDROID_SODIUM_LIB=$WORK_DIR/libzmq-android/libsodium/libsodium_x86_64/lib; echo "ANDROID_SODIUM_LIB: $ANDROID_SODIUM_LIB"
# export SODIUM_LIB_DIR=$ANDROID_SODIUM_LIB; echo "SODIUM_LIB_DIR: $SODIUM_LIB_DIR"
# export ANDROID_ZMQ_LIB=$WORK_DIR/libzmq-android/zmq/libzmq_x86_64/lib; echo "ANDROID_ZMQ_LIB: $ANDROID_ZMQ_LIB"
# #export LIBZMQ_LIB_DIR=$ANDROID_ZMQ_LIB; echo "LIBZMQ_LIB_DIR: $LIBZMQ_LIB_DIR"
# #export LIBZMQ_INCLUDE_DIR=$WORK_DIR/libzmq-android/zmq/libzmq_x86_64/include; echo "LIBZMQ_INCLUDE_DIR: $LIBZMQ_INCLUDE_DIR"
# export ANDROID_SQLITE_LIB=$WORK_DIR/libsqlite3-android/sqlite3-android/obj/local/x86_64; echo "ANDROID_SQLITE_LIB: $ANDROID_SQLITE_LIB"
# sed -i .bak 's/\"\"\.as_ptr() as \*const u8/\"\"\.as_ptr() as \*const i8/' src/services/wallet/storage/plugged/mod.rs
# cargo build --target x86_64-linux-android --release --verbose
# echo "-----------------------------------------------------------------------------------------------"

# This builds the library for code that runs in OSX
export OPENSSL_DIR=$OPENSSL_DIR_DARWIN
#unset LIBZMQ_LIB_DIR
#unset LIBZMQ_INCLUDE_DIR
unset ANDROID_SODIUM_LIB
SODIUM_LIB_DIR=/usr/local/lib
ANDROID_ZMQ_LIB=/usr/local/lib
unset ANDROID_SQLITE_LIB
sed -i .bak 's/\"\"\.as_ptr() as \*const u8/\"\"\.as_ptr() as \*const i8/' src/services/wallet/storage/plugged/mod.rs
# KS:Commenting this for now, because it fails for osx build
# cargo build --target x86_64-apple-darwin --release --verbose
echo "-----------------------------------------------------------------------------------------------"

#########################################################################################################################
# Now setup libsovtoken
#########################################################################################################################

if [ -d $WORK_DIR/libsovtoken-android ]; then
    echo "libsovtoken build for android already exist"
else
    mkdir -p $WORK_DIR/libsovtoken-android
    cd $WORK_DIR/libsovtoken-android
    curl -o libsovtoken-android.zip $LIBSOVTOKEN_ANDROID_BUILD_URL
    unzip libsovtoken-android.zip
    # Deletes extra folders we don't need
    rm libsovtoken-android.zip
    rm -rf __MACOSX
fi

# This builds the library for code that runs in OSX
export OPENSSL_DIR=$OPENSSL_DIR_DARWIN
unset ANDROID_SODIUM_LIB
SODIUM_LIB_DIR=/usr/local/lib
ANDROID_ZMQ_LIB=/usr/local/lib
unset ANDROID_SQLITE_LIB

export PATH=$ORIGINAL_PATH
