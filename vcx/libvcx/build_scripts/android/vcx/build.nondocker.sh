#!/usr/bin/env bash

WORKDIR=${PWD}
TARGET_ARCH=$1
TARGET_API=$2
CROSS_COMPILE=$3

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    echo "Sample : ./build.nondocker.sh x86 16 i686-linux-android openssl_x86 libsodium_x86 libzmq_x86 libindy"
    exit 1
fi

if [ -z "${TARGET_API}" ]; then
    echo STDERR "Missing TARGET_API argument"
    echo STDERR "e.g. 21"
    echo "Sample : ./build.nondocker.sh x86 16 i686-linux-android openssl_x86 libsodium_x86 libzmq_x86 libindy"
    exit 1
fi

if [ -z "${CROSS_COMPILE}" ]; then
    echo STDERR "Missing CROSS_COMPILE argument"
    echo STDERR "e.g. i686-linux-android"
    echo "Sample : ./build.nondocker.sh x86 16 i686-linux-android openssl_x86 libsodium_x86 libzmq_x86 libindy"
    exit 1
fi


if [ -z "${OPENSSL_DIR}" ]; then
    OPENSSL_DIR="openssl_${TARGET_ARCH}"
    if [ -d "${OPENSSL_DIR}" ] ; then
        echo "Found ${OPENSSL_DIR}"
    elif [ -z "$4" ]; then
        echo STDERR "Missing OPENSSL_DIR argument and environment variable"
        echo STDERR "e.g. set OPENSSL_DIR=<path> for environment or openssl_${TARGET_ARCH}"
        exit 1
    else
        OPENSSL_DIR=$4
    fi
fi

if [ -z "${SODIUM_DIR}" ] ; then
    SODIUM_DIR="libsodium_${TARGET_ARCH}"
    if [ -d "${SODIUM_DIR}" ] ; then
        echo "Found ${SODIUM_DIR}"
    elif [ -z "$5" ]; then
        echo STDERR "Missing SODIUM_DIR argument and environment variable"
        echo STDERR "e.g. set SODIUM_DIR=<path> for environment or libsodium_${TARGET_ARCH}"
        exit 1
    else
        SODIUM_DIR=$5
    fi
fi

if [ -z "${LIBZMQ_DIR}" ] ; then
    LIBZMQ_DIR="libzmq_${TARGET_ARCH}"
    if [ -d "${LIBZMQ_DIR}" ] ; then
        echo "Found ${LIBZMQ_DIR}"
    elif [ -z "$6" ]; then
        echo STDERR "Missing LIBZMQ_DIR argument and environment variable"
        echo STDERR "e.g. set LIBZMQ_DIR=<path> for environment or libzmq_${TARGET_ARCH}"
        exit 1
    else
        LIBZMQ_DIR=$6
    fi
fi

if [ -z "${LIBINDY_DIR}" ] ; then
    LIBINDY_DIR="libindy_${TARGET_ARCH}"
    if [ -d "${LIBINDY_DIR}" ] ; then
        echo "Found ${LIBINDY_DIR}"
    elif [ -z "$7" ] ; then
        echo STDERR "Missing LIBINDY_DIR argument and environment variable"
        echo STDERR "e.g. set LIBINDY_DIR=<path> for environment or libindy_${TARGET_ARCH}"
        exit 1
    else
        LIBINDY_DIR=$7
    fi
fi

if [ -z "${LIBNULLPAY_DIR}" ] ; then
    LIBNULLPAY_DIR="libnullpay_${TARGET_ARCH}"
    if [ -d "${LIBNULLPAY_DIR}" ] ; then
        echo "Found ${LIBNULLPAY_DIR}"
    elif [ -z "$8" ] ; then
        echo STDERR "Missing LIBNULLPAY_DIR argument and environment variable"
        echo STDERR "e.g. set LIBNULLPAY_DIR=<path> for environment or libnullpay_${TARGET_ARCH}"
        exit 1
    else
        LIBNULLPAY_DIR=$8
    fi
fi


if [ "$(uname)" == "Darwin" ]; then
    echo "Downloading NDK for OSX"
    export TOOLCHAIN_PREFIX=${WORKDIR}/toolchains/darwin
    mkdir -p ${TOOLCHAIN_PREFIX}
    pushd $TOOLCHAIN_PREFIX
    if [ ! -d "android-ndk-r16b" ] ; then
        echo "Downloading android-ndk-r16b-darwin-x86_64.zip"
        wget -q https://dl.google.com/android/repository/android-ndk-r16b-darwin-x86_64.zip
        unzip -qq android-ndk-r16b-darwin-x86_64.zip
    else
        echo "Skipping download android-ndk-r16b-linux-x86_64.zip"
    fi
    export ANDROID_NDK_ROOT=${TOOLCHAIN_PREFIX}/android-ndk-r16b
    popd
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    echo "Downloading NDK for Linux"
    export TOOLCHAIN_PREFIX=${WORKDIR}/toolchains/linux
    mkdir -p ${TOOLCHAIN_PREFIX}
    pushd $TOOLCHAIN_PREFIX
    if [ ! -d "android-ndk-r16b" ] ; then
        echo "Downloading android-ndk-r16b-linux-x86_64.zip"
        wget -q https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip
        unzip -qq android-ndk-r16b-linux-x86_64.zip
    else
        echo "Skipping download android-ndk-r16b-linux-x86_64.zip"
    fi
    export ANDROID_NDK_ROOT=${TOOLCHAIN_PREFIX}/android-ndk-r16b
    popd
fi

#LIBVCX=${WORKDIR}/sdk/vcx/libvcx/
#cp -rf ./../../../../../vcx/libvcx/include ${LIBVCX}
#cp -rf ./../../../../../vcx/libvcx/scripts ${LIBVCX}
#cp -rf ./../../../../../vcx/libvcx/src ${LIBVCX}
#cp -rf ./../../../../../vcx/libvcx/build.rs ${LIBVCX}
#cp -rf ./../../../../../vcx/libvcx/Cargo.toml ${LIBVCX}
LIBVCX=../../../

export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_INCREMENTAL=1
export RUST_LOG=indy=trace
export RUST_TEST_THREADS=1
export RUST_BACKTRACE=1
export OPENSSL_DIR=${WORKDIR}/${OPENSSL_DIR}
export SODIUM_LIB_DIR=${WORKDIR}/${SODIUM_DIR}/lib
export SODIUM_INCLUDE_DIR=${WORKDIR}/${SODIUM_DIR}/include
export LIBZMQ_LIB_DIR=${WORKDIR}/${LIBZMQ_DIR}/lib
export LIBZMQ_INCLUDE_DIR=${WORKDIR}/${LIBZMQ_DIR}/include
export LIBINDY_DIR=${WORKDIR}/${LIBINDY_DIR}
export LIBNULLPAY_DIR=${WORKDIR}/${LIBNULLPAY_DIR}
export TOOLCHAIN_DIR=${TOOLCHAIN_PREFIX}/${TARGET_ARCH}
export PATH=${TOOLCHAIN_DIR}/bin:${PATH}
export PKG_CONFIG_ALLOW_CROSS=1
export CC=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang
export AR=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ar
export CXX=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang++
export CXXLD=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ld
export RANLIB=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ranlib
export TARGET=android

printenv

python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${TARGET_ARCH} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR}
cat << EOF > ~/.cargo/config
[target.${CROSS_COMPILE}]
ar = "${AR}"
linker = "${CC}"
EOF

rustup target add ${CROSS_COMPILE}

pushd $LIBVCX
export OPENSSL_STATIC=1
cargo clean
cargo build --release --target=${CROSS_COMPILE}
popd

LIBVCX_BUILDS=${WORKDIR}/libvcx_${TARGET_ARCH}
mkdir -p ${LIBVCX_BUILDS}
$CC -v -shared -o ${LIBVCX_BUILDS}/libvcx.so -Wl,--whole-archive \
${LIBVCX}/target/${CROSS_COMPILE}/release/libvcx.a \
${TOOLCHAIN_DIR}/sysroot/usr/lib/libz.so \
${TOOLCHAIN_DIR}/sysroot/usr/lib/libm.a \
${TOOLCHAIN_DIR}/sysroot/usr/lib/liblog.so \
${LIBINDY_DIR}/libindy.a \
${LIBNULLPAY_DIR}/libnullpay.a \
${TOOLCHAIN_DIR}/${CROSS_COMPILE}/lib/libgnustl_shared.so \
${OPENSSL_DIR}/lib/libssl.a \
${OPENSSL_DIR}/lib/libcrypto.a \
${SODIUM_LIB_DIR}/libsodium.a \
${LIBZMQ_LIB_DIR}/libzmq.a \
${TOOLCHAIN_DIR}/${CROSS_COMPILE}/lib/libgnustl_shared.so -Wl,--no-whole-archive -z muldefs
cp "${LIBVCX}/target/${CROSS_COMPILE}/release/libvcx.a" ${LIBVCX_BUILDS}/

