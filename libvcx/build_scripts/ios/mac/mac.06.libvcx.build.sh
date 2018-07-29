#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

INDY_SDK=$WORK_DIR/vcx-indy-sdk
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

source ./mac.05.libvcx.env.sh
cd ../../..
DEBUG_SYMBOLS="debuginfo"

if [ ! -z "$1" ]; then
    DEBUG_SYMBOLS=$1
fi

if [ "$DEBUG_SYMBOLS" = "nodebug" ]; then
    sed -i .bak 's/debug = true/debug = false/' Cargo.toml
fi

IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
if [ ! -z "$2" ]; then
    IOS_TARGETS=$2
fi

CLEAN_BUILD="cleanbuild"
if [ ! -z "$3" ]; then
    CLEAN_BUILD=$3
fi

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    cargo clean
    # cargo update
fi

git log -1 > $WORK_DIR/evernym.vcx-sdk.git.commit.log

export OPENSSL_LIB_DIR_DARWIN=$OPENSSL_LIB_DIR

bkpIFS="$IFS"
IFS=',()][' read -r -a targets <<<"${IOS_TARGETS}"
echo "Building targets: ${targets[@]}"    ##Or printf "%s\n" ${array[@]}
IFS="$bkpIFS"

to_combine=""
for target in ${targets[*]}
do
    if [ "${target}" = "aarch64-apple-ios" ]; then
        target_arch="arm64"
    elif [ "${target}" = "armv7-apple-ios" ]; then
        target_arch="armv7"
    elif [ "${target}" = "armv7s-apple-ios" ]; then
        target_arch="armv7s"
    elif [ "${target}" = "i386-apple-ios" ]; then
        target_arch="i386"
    elif [ "${target}" = "x86_64-apple-ios" ]; then
        target_arch="x86_64"
    fi

    export OPENSSL_LIB_DIR=$WORK_DIR/OpenSSL-for-iPhone/lib/${target_arch}
    export IOS_SODIUM_LIB=$WORK_DIR/libzmq-ios/libsodium-ios/dist/ios/lib/${target_arch}
    export IOS_ZMQ_LIB=$WORK_DIR/libzmq-ios/dist/ios/lib/${target_arch}
    export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/${target}/release
    export LIBNULLPAY_DIR=$WORK_DIR/vcx-indy-sdk/libnullpay/target/${target}/release
    # export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/${target}/debug
    # export LIBNULLPAY_DIR=$WORK_DIR/vcx-indy-sdk/libnullpay/target/${target}/debug
    #export LIBINDY_DIR=$WORK_DIR/vcx-indy-sdk/libindy/target/universal/release
    #export LIBNULLPAY_DIR=$WORK_DIR/vcx-indy-sdk/libnullpay/target/universal/release

    # To build for macos
    #cargo build
    #export LIBINDY_DIR=/usr/local/lib
    #export RUST_BACKTRACE=1
    # To build for iOS
    #LIBINDY_DIR=/usr/local/lib RUST_BACKTRACE=1 cargo lipo --release
    #cargo lipo --release --verbose --targets="${IOS_TARGETS}"
    
    # if [ -f "./target/universal/release/libvcx.a" ]; then
    #     mv ./target/universal/release/libvcx.a ./libvcx.previous.a
    # fi

    #rm ./target/universal/release/libvcx.a
    #cargo lipo --release --verbose --targets="${target}"
    cargo build --target "${target}" --release --verbose
    # cargo build --release --target "${target}"

    # if [ -f "./libvcx.previous.a" ]; then
    #     lipo -create -output ./combined.ios.libvcx.a ./target/universal/release/libvcx.a ./libvcx.previous.a
    #     mv ./combined.ios.libvcx.a ./target/universal/release/libvcx.a
    #     rm ./libvcx.previous.a
    # fi
    to_combine="${to_combine} ./target/${target}/release/libvcx.a"

done
#rm ./target/universal/release/libvcx.a
mkdir -p ./target/universal/release
lipo -create $to_combine -o ./target/universal/release/libvcx.a
#lipo -create -output ./combined.ios.libvcx.a ./target/universal/release/libvcx.a ./libvcx.previous.a

export OPENSSL_LIB_DIR=$OPENSSL_LIB_DIR_DARWIN

#cargo test

#lipo -info 
