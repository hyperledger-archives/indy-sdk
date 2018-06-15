#!/usr/bin/env bash

#!/usr/bin/env bash

WORKDIR=${PWD}
TARGET_ARCH=$1
TARGET_API=$2
CROSS_COMPILE=$3
export INDY_DIR=$4

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    echo "Sample : ./build.nondocker.sh x86 16 i686-linux-android <ABSOLUTE_PATH_TO_LIBINDY_BINARIES_DIR>"
    exit 1
fi

if [ -z "${TARGET_API}" ]; then
    echo STDERR "Missing TARGET_API argument"
    echo STDERR "e.g. 21"
    echo "Sample : ./build.nondocker.sh x86 16 i686-linux-android <ABSOLUTE_PATH_TO_LIBINDY_BINARIES_DIR>"
    exit 1
fi

if [ -z "${CROSS_COMPILE}" ]; then
    echo STDERR "Missing CROSS_COMPILE argument"
    echo STDERR "e.g. i686-linux-android"
    echo "Sample : ./build.nondocker.sh x86 16 i686-linux-android <ABSOLUTE_PATH_TO_LIBINDY_BINARIES_DIR>"
    exit 1
fi

if [ -z "${INDY_DIR}" ]; then
    echo STDERR "Missing INDY_DIR argument"
    echo STDERR "Should have path to directory containing libindy binaries"
    echo "Sample : ./build.nondocker.sh x86 16 i686-linux-android <ABSOLUTE_PATH_TO_LIBINDY_BINARIES_DIR>"
    exit 1
fi




if [ "$(uname)" == "Darwin" ]; then
    echo "Downloading NDK for OSX"
    export TOOLCHAIN_PREFIX=${WORKDIR}/toolchains/darwin
    mkdir -p ${TOOLCHAIN_PREFIX}
    pushd $TOOLCHAIN_PREFIX
    if [ ! -d "android-ndk-r16b" ] ; then
        echo "Downloading android-ndk-r16b-darwin-x86_64.zip"
        wget https://dl.google.com/android/repository/android-ndk-r16b-darwin-x86_64.zip
        unzip -qq android-ndk-r16b-darwin-x86_64.zip
    else
        echo "Skipping download android-ndk-r16b-linux-x86_64.zip"
    fi
    export ANDROID_NDK_ROOT=${TOOLCHAIN_PREFIX}/android-ndk-r16b
    popd
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    echo "Downloading NDK for Linux"
    export TOOLCHAIN_PREFIX=${WORKDIR}/toolchains/linux
    mkdir ${TOOLCHAIN_PREFIX}
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



export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_INCREMENTAL=1
export RUST_LOG=indy=trace
export RUST_TEST_THREADS=1
export RUST_BACKTRACE=1
export TOOLCHAIN_DIR=${TOOLCHAIN_PREFIX}/${TARGET_ARCH}
export PATH=${TOOLCHAIN_DIR}/bin:${PATH}
export PKG_CONFIG_ALLOW_CROSS=1
export CC=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang
export AR=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ar
export CXX=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang++
export CXXLD=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ld
export RANLIB=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ranlib
export TARGET=android

python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${TARGET_ARCH} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR}
cat << EOF > ~/.cargo/config
[target.${CROSS_COMPILE}]
ar = "${AR}"
linker = "${CC}"
EOF

rustup target add ${CROSS_COMPILE}

#BUILD LIBNULLPAY
LIBNULLPAY_SRC=${WORKDIR}/indy-sdk/libnullpay
mkdir -p $LIBNULLPAY_SRC
cp -rf ./../../../libnullpay/build.rs ${LIBNULLPAY_SRC}
cp -rf ./../../../libnullpay/src ${LIBNULLPAY_SRC}
cp -rf ./../../../libnullpay/include ${LIBNULLPAY_SRC}
cp -rf ./../../../libnullpay/Cargo.toml ${LIBNULLPAY_SRC}
pushd $LIBNULLPAY_SRC

cargo clean
cargo build --release --target=${CROSS_COMPILE} --verbose
popd

LIBNULLPAY_BUILDS=${WORKDIR}/libnullpay_${TARGET_ARCH}
mkdir -p ${LIBNULLPAY_BUILDS}
cp "${LIBNULLPAY_SRC}/target/${CROSS_COMPILE}/release/libnullpay.so" ${LIBNULLPAY_BUILDS}/
cp "${LIBNULLPAY_SRC}/target/${CROSS_COMPILE}/release/libnullpay.a" ${LIBNULLPAY_BUILDS}/