#!/usr/bin/env bash

WORKDIR=${PWD}
DOWNLOAD_PREBUILTS="0"
BUILD_FOLDER=${WORKDIR}/android_build

while getopts ":d" opt; do
    case ${opt} in
        d) DOWNLOAD_PREBUILTS="1";;
        \?);;
    esac
done
shift $((OPTIND -1))

TARGET_ARCH=$1
#TARGET_API=$2
#CROSS_COMPILE=$3
if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi
#if [ -z "${TARGET_API}" ]; then
#    echo STDERR "Missing TARGET_API argument"
#    echo STDERR "e.g. 21"
#    exit 1
#fi
#
#if [ -z "${CROSS_COMPILE}" ]; then
#    echo STDERR "Missing CROSS_COMPILE argument"
#    echo STDERR "e.g. i686-linux-android"
#    exit 1
#fi


build_for_arch(){
    if [ -z $1 ]; then
        echo "please provide the arch e.g arm, x86 or arm64"
        exit 1
    fi
    if [ $1 == "arm" ]; then
        export TARGET_ARCH="arm"
        export TARGET_API="16"
        export TRIPLET="arm-linux-androideabi"
        export ABI="armeabi-v7a"
        execute_build_steps
    fi

    if [ $1 == "arm64" ]; then
        export ARCH="arm64"
        export TARGET_API="21"
        export TRIPLET="aarch64-linux-android"
        export ABI="arm64-v8a"
        execute_build_steps
    fi

    if [ $1 == "x86" ]; then
        export ARCH="x86"
        export TARGET_API="16"
        export TRIPLET="i686-linux-android"
        export ABI="x86"
        execute_build_steps
    fi
    if [ $1 == "all" ]; then
        export TARGET_ARCH="arm"
        export TARGET_API="16"
        export TRIPLET="arm-linux-androideabi"
        export ABI="armeabi-v7a"
        execute_build_steps

        export ARCH="arm64"
        export TARGET_API="21"
        export TRIPLET="aarch64-linux-android"
        export ABI="arm64-v8a"
        execute_build_steps

        export ARCH="x86"
        export TARGET_API="16"
        export TRIPLET="i686-linux-android"
        export ABI="x86"
        execute_build_steps
    fi

}

download_and_unzip_dependencies_for_all_architectures(){
    #TODO Get dependencies in more optimized way
    pushd ${BUILD_FOLDER}
        if [ ! -d "indy-android-dependencies" ] ; then
            git clone https://github.com/evernym/indy-android-dependencies.git
            pushd indy-android-dependencies/prebuilt/
                git checkout tags/v1.0.1
                find . -name "*.zip" | xargs -P 5 -I FILENAME sh -c 'unzip -o -qq -d "$(dirname "FILENAME")" "FILENAME"'
            popd
            ln -sf indy-android-dependencies/prebuilt dependencies
        fi
        export OPENSSL_DIR=${BUILD_FOLDER}/dependencies/openssl/openssl_${TARGET_ARCH}
        export SODIUM_DIR=${BUILD_FOLDER}/dependencies/sodium/libsodium_${TARGET_ARCH}
        export LIBZMQ_DIR=${BUILD_FOLDER}/dependencies/zmq/libzmq_${TARGET_ARCH}
	popd
}


setup_dependencies(){
    if [ "${DOWNLOAD_PREBUILTS}" == "1" ]; then
        download_and_unzip_dependencies_for_all_architectures
        else
            echo "not downloading prebuilt dependencies. Dependencies locations have to be passed"
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

            if [ -z "${SODIUM_DIR}" ]; then
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
                elif [ -z "$6" ] ; then
                    echo STDERR "Missing LIBZMQ_DIR argument and environment variable"
                    echo STDERR "e.g. set LIBZMQ_DIR=<path> for environment or libzmq_${TARGET_ARCH}"
                    exit 1
                else
                    LIBZMQ_DIR=$6
                fi
            fi


    fi
}



download_and_setup_toolchain(){
    if [ "$(uname)" == "Darwin" ]; then
        echo "Downloading NDK for OSX"
        export TOOLCHAIN_PREFIX=${BUILD_FOLDER}/toolchains/darwin
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
        export TOOLCHAIN_PREFIX=${BUILD_FOLDER}/toolchains/linux
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
}




set_env_vars(){
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
    export TOOLCHAIN_DIR=${TOOLCHAIN_PREFIX}/${TARGET_ARCH}
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





#LIBINDY_SRC=${WORKDIR}/indy-sdk/libindy
#mkdir -p ${LIBINDY_SRC}
#mkdir -p ${LIBINDY_SRC}/.cargo
##TODO FIX WRITING BELOW CONFIG
#cat << EOF > ${LIBINDY_SRC}/.cargo/config
#[target.${TRIPLET}]
#ar = "${AR}"
#linker = "${CC}"
#runner = "./run-on-adroid.sh"
#EOF

#cp -rf ./../../.cargo ${LIBINDY_SRC}
#cp -rf ./../../build.rs ${LIBINDY_SRC}
#cp -rf ./../../src ${LIBINDY_SRC}
#cp -rf ./../../include ${LIBINDY_SRC}
#cp -rf ./../../Cargo.toml ${LIBINDY_SRC}
#cp -rf ./../../run-on-android.sh ${LIBINDY_SRC}



#LIBINDY_BUILDS=${WORKDIR}/libindy_${TARGET_ARCH}
#mkdir -p ${LIBINDY_BUILDS}
statically_link_dependencies_with_libindy(){
    $CC -v -shared -o${BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib/libindy.so -Wl,--whole-archive \
        ${WORKDIR}/target/${TRIPLET}/release/libindy.a \
        ${TOOLCHAIN_DIR}/sysroot/usr/lib/libz.so \
        ${TOOLCHAIN_DIR}/sysroot/usr/lib/libm.a \
        ${TOOLCHAIN_DIR}/sysroot/usr/lib/liblog.so \
        ${OPENSSL_DIR}/lib/libssl.a \
        ${OPENSSL_DIR}/lib/libcrypto.a \
        ${SODIUM_LIB_DIR}/libsodium.a \
        ${LIBZMQ_LIB_DIR}/libzmq.a \
        ${TOOLCHAIN_DIR}/${TRIPLET}/lib/libgnustl_shared.so \
        -Wl,--no-whole-archive -z muldefs
}

package_library(){
    mkdir -p ${BUILD_FOLDER}/libindy_${TARGET_ARCH}/include
    mkdir -p ${BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib

    cp "${WORKDIR}/target/${TRIPLET}/release/libindy.a" ${BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib
    cp "${WORKDIR}/target/${TRIPLET}/release/libindy.so" ${BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib
    mv "${BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib/libindy.so" "${BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib/libindy_shared.so"
    statically_link_dependencies_with_libindy
}

build(){
    pushd ${WORKDIR}
        cargo clean
        RUSTFLAGS="-L${TOOLCHAIN_DIR}/i686-linux-android/lib -lgnustl_shared" \
            cargo build --target=${TRIPLET} -v
    popd
}

test(){
    pushd ${WORKDIR}
        cargo clean
        RUSTFLAGS="-L${TOOLCHAIN_DIR}/i686-linux-android/lib -lgnustl_shared" \
            cargo test --target=${TRIPLET} -v
    popd
}

cleanup(){
    rm -rf ${BUILD_FOLDER}/indy-android-dependencies
}

execute_build_steps(){
        setup_dependencies
        download_and_setup_toolchain
        set_env_vars
        create_standalone_toolchain_and_rust_target
        build
        test
        package_library
}

build_for_arch $@
