#!/usr/bin/env bash

CI_DIR=$(cd `dirname $0` && pwd)
INDY_WORKDIR="$(realpath "${CI_DIR}/..")"
ANDROID_BUILD_FOLDER="$(realpath "${CI_DIR}/../android_build")"
export TOOLCHAIN_PREFIX=${ANDROID_BUILD_FOLDER}/toolchains
## set this variable to 1 if you want to download the prebuilt binaries


TARGET_ARCH=$1

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi

download_adb(){
    mkdir -p ${ANDROID_BUILD_FOLDER}/adb
    pushd ${ANDROID_BUILD_FOLDER}/adb

        wget https://dl.google.com/android/repository/platform-tools_r28.0.0-linux.zip
        unzip -qq platform-tools_r28.0.0-linux.zip
    popd
}

download_sdk(){
    mkdir -p ${ANDROID_BUILD_FOLDER}/sdk
     pushd ${ANDROID_BUILD_FOLDER}/adb

        wget https://dl.google.com/android/repository/sdk-tools-linux-4333796.zip
        unzip -qq sdk-tools-linux-4333796.zip
    popd
}

generate_arch_flags(){
    if [ -z $1 ]; then
        echo "please provide the arch e.g arm, x86 or arm64"
        exit 1
    fi
    if [ $1 == "arm" ]; then
        export TARGET_ARCH="arm"
        export TARGET_API="16"
        export TRIPLET="arm-linux-androideabi"
        export ABI="armeabi-v7a"
    fi

    if [ $1 == "arm64" ]; then
        export ARCH="arm64"
        export TARGET_API="21"
        export TRIPLET="aarch64-linux-android"
        export ABI="arm64-v8a"
    fi

    if [ $1 == "x86" ]; then
        export ARCH="x86"
        export TARGET_API="16"
        export TRIPLET="i686-linux-android"
        export ABI="x86"
    fi

}

download_and_unzip_dependencies_for_all_architectures(){
    #TODO Get dependencies in more optimized way
    pushd ${ANDROID_BUILD_FOLDER}
        if [ ! -d "indy-android-dependencies" ] ; then
            git clone https://github.com/evernym/indy-android-dependencies.git
            pushd ${ANDROID_BUILD_FOLDER}/indy-android-dependencies/prebuilt/
#                git checkout tags/v1.0.1
                find . -name "*.zip" | xargs -P 5 -I FILENAME sh -c 'unzip -o -qq -d "$(dirname "FILENAME")" "FILENAME"'
            popd
        fi
        export OPENSSL_DIR=${ANDROID_BUILD_FOLDER}/indy-android-dependencies/prebuilt/openssl/openssl_${TARGET_ARCH}
        export SODIUM_DIR=${ANDROID_BUILD_FOLDER}/indy-android-dependencies/prebuilt/sodium/libsodium_${TARGET_ARCH}
        export LIBZMQ_DIR=${ANDROID_BUILD_FOLDER}/indy-android-dependencies/prebuilt/zmq/libzmq_${TARGET_ARCH}
	popd
}



create_standalone_toolchain_and_rust_target(){
    #will only create toolchain if not already created
    python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py \
    --arch ${TARGET_ARCH} \
    --api ${TARGET_API} \
    --stl=gnustl \
    --install-dir ${TOOLCHAIN_DIR}

    # add rust target
    rustup target add ${TRIPLET}
}

download_and_setup_toolchain(){
    if [ "$(uname)" == "Darwin" ]; then
        echo "Downloading NDK for OSX"
        export TOOLCHAIN_PLATFORM_PREFIX=${TOOLCHAIN_PREFIX}/darwin
        mkdir -p ${TOOLCHAIN_PLATFORM_PREFIX}
        pushd ${TOOLCHAIN_PLATFORM_PREFIX}
            if [ ! -d "android-ndk-r16b" ] ; then
                echo "Downloading android-ndk-r16b-darwin-x86_64.zip"
                wget https://dl.google.com/android/repository/android-ndk-r16b-darwin-x86_64.zip
                unzip -qq android-ndk-r16b-darwin-x86_64.zip
            else
                echo "Skipping download android-ndk-r16b-darwin-x86_64.zip"
            fi
        popd
        pushd ${TOOLCHAIN_PREFIX}
            ln -nsf $(realpath ${TOOLCHAIN_PLATFORM_PREFIX}) standalone_toolchains
            export ANDROID_NDK_ROOT=${TOOLCHAIN_PREFIX}/standalone_toolchains/android-ndk-r16b
        popd

    elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
        echo "Downloading NDK for Linux"
        export TOOLCHAIN_PLATFORM_PREFIX=${TOOLCHAIN_PREFIX}/linux
        mkdir -p ${TOOLCHAIN_PLATFORM_PREFIX}
        pushd ${TOOLCHAIN_PLATFORM_PREFIX}
            if [ ! -d "android-ndk-r16b" ] ; then
                echo "Downloading android-ndk-r16b-linux-x86_64.zip"
                wget -q https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip
                unzip -qq android-ndk-r16b-linux-x86_64.zip
            else
                echo "Skipping download android-ndk-r16b-linux-x86_64.zip"
            fi
            export ANDROID_NDK_ROOT=${TOOLCHAIN_PLATFORM_PREFIX}/android-ndk-r16b
        popd

        pushd ${TOOLCHAIN_PREFIX}
            ln -nsf $(realpath ${TOOLCHAIN_PLATFORM_PREFIX}) standalone_toolchains
            export ANDROID_NDK_ROOT=${TOOLCHAIN_PREFIX}/standalone_toolchains/android-ndk-r16b
        popd
    fi

}

set_env_vars(){
    export PKG_CONFIG_ALLOW_CROSS=1
    export CARGO_INCREMENTAL=1
    export RUST_LOG=indy=trace
    export RUST_TEST_THREADS=1
    export RUST_BACKTRACE=1
    export OPENSSL_DIR=${OPENSSL_DIR}
    export SODIUM_LIB_DIR=${SODIUM_DIR}/lib
    export SODIUM_INCLUDE_DIR=${SODIUM_DIR}/include
    export LIBZMQ_LIB_DIR=${LIBZMQ_DIR}/lib
    export LIBZMQ_INCLUDE_DIR=${LIBZMQ_DIR}/include
    export TOOLCHAIN_DIR=${TOOLCHAIN_PREFIX}/standalone_toolchains/${TARGET_ARCH}
    export PATH=${TOOLCHAIN_DIR}/bin:${PATH}
    export PKG_CONFIG_ALLOW_CROSS=1
    export CC=${TOOLCHAIN_DIR}/bin/${TRIPLET}-clang
    export AR=${TOOLCHAIN_DIR}/bin/${TRIPLET}-ar
    export CXX=${TOOLCHAIN_DIR}/bin/${TRIPLET}-clang++
    export CXXLD=${TOOLCHAIN_DIR}/bin/${TRIPLET}-ld
    export RANLIB=${TOOLCHAIN_DIR}/bin/${TRIPLET}-ranlib
    export TARGET=android
    export OPENSSL_STATIC=1
}
