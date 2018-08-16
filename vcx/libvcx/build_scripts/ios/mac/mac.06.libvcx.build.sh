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

BUILD_CACHE=~/.build_libvxc/ioscache
mkdir -p ${BUILD_CACHE}

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    cargo clean
    rm -rf ${BUILD_CACHE}/target
    # cargo update
else
    if [ -d ${BUILD_CACHE}/target ]; then
        echo "Optimizing iOS build using folder: $(abspath ${BUILD_CACHE}/target)"
        cp -rfp ${BUILD_CACHE}/target .
    fi
fi

git log -1 > $WORK_DIR/evernym.vcx-sdk.git.commit.log

# change libvcx to use libsovtoken feature
sed -i .bak 's/"nullpay"/"sovtoken"/' Cargo.toml

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

    libtool="/usr/bin/libtool"
    libsovtoken_dir="${WORK_DIR}/libsovtoken-ios/libsovtoken"
    libindy_dir="${WORK_DIR}/libindy"

    if [ -d ${libsovtoken_dir}/${target_arch} ]; then
        echo "${target_arch} libsovtoken architecture already extracted"
    else
        mkdir -p ${libsovtoken_dir}/${target_arch}
        lipo -extract $target_arch ${libsovtoken_dir}/universal/libsovtoken.a -o ${libsovtoken_dir}/${target_arch}/libsovtoken.a
        ${libtool} -static ${libsovtoken_dir}/${target_arch}/libsovtoken.a -o ${libsovtoken_dir}/${target_arch}/libsovtoken_libtool.a
        mv ${libsovtoken_dir}/${target_arch}/libsovtoken_libtool.a ${libsovtoken_dir}/${target_arch}/libsovtoken.a
    fi

    if [ -d ${libindy_dir}/${target_arch} ]; then
        echo "${target_arch} libindy architecture already extracted"
    else
        mkdir -p ${libindy_dir}/${target_arch}
        lipo -extract $target_arch ${libindy_dir}/libindy.a -o ${libindy_dir}/${target_arch}/libindy.a
        ${libtool} -static ${libindy_dir}/${target_arch}/libindy.a -o ${libindy_dir}/${target_arch}/libindy_libtool.a
        mv ${libindy_dir}/${target_arch}/libindy_libtool.a ${libindy_dir}/${target_arch}/libindy.a
    fi

    export OPENSSL_LIB_DIR=$WORK_DIR/OpenSSL-for-iPhone/lib/${target_arch}
    export IOS_SODIUM_LIB=$WORK_DIR/libzmq-ios/libsodium-ios/dist/ios/lib/${target_arch}
    export IOS_ZMQ_LIB=$WORK_DIR/libzmq-ios/dist/ios/lib/${target_arch}
    export LIBINDY_DIR=${libindy_dir}/${target_arch}
    #export LIBNULLPAY_DIR=$WORK_DIR/vcx-indy-sdk/libnullpay/target/${target}/release
    export LIBSOVTOKEN_DIR=${libsovtoken_dir}/${target_arch}

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
    cargo build --target "${target}" --release
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

echo "Copying iOS target folder into directory: $(abspath "${BUILD_CACHE}")"
cp -rfp ./target ${BUILD_CACHE}

export OPENSSL_LIB_DIR=$OPENSSL_LIB_DIR_DARWIN

#cargo test

#lipo -info
