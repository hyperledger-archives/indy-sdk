#!/bin/bash

setup() {
    echo "Working Directory: ${PWD}"
    set -e
    export ARCH=$1

    export PATH=${HOME}/.cargo/bin:${PATH}
    export PATH=$PATH:/opt/gradle/gradle-3.4.1/bin
    export PATH=${PATH}:$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools:$ANDROID_HOME/build-tools/25.0.2/
    source /etc/profile
	if [ ! -d runtime_android_build ]; then
        mkdir runtime_android_build
    fi
    cd runtime_android_build
	retrieve_prebuilt_binaries
	generate_flags $1
    if [ ! -d "toolchains" ]; then
        mkdir toolchains
    fi

    # For Jenkins (MAIN)
    ANDROID_JNI_LIB=../vcx/wrappers/java/vcx/src/main/jniLibs

    # For docker 
    #ANDROID_JNI_LIB=~/vcx/wrappers/java/vcx/src/main/jniLibs

    mkdir -p ${ANDROID_JNI_LIB}/arm
    mkdir -p ${ANDROID_JNI_LIB}/x86
    mkdir -p ${ANDROID_JNI_LIB}/arm64
}

copy_dependencies() {
    PATH_TO_CP=$1
    if [ ! -d ${PATH_TO_CP}/toolchains/linux ]; then
        cp -rf toolchains ${PATH_TO_CP}
    fi
    cp -rf openssl_${ARCH} ${PATH_TO_CP}
    cp -rf libsodium_${ARCH} ${PATH_TO_CP}
    cp -rf libzmq_${ARCH} ${PATH_TO_CP}
}

retrieve_prebuilt_binaries() {
    PREBUILT_LINK=https://github.com/faisal00813/indy-android-dependencies/raw/master/prebuilt

    if [ ! -d "openssl_${ARCH}" ]; then
        echo "retrieving openssl prebuilt library"
        wget -q ${PREBUILT_LINK}/openssl/openssl_${ARCH}.zip
        unzip -qq openssl_${ARCH}.zip
    fi

    if [ ! -d "libsodium_${ARCH}" ]; then
        echo "retrieving libsodium prebuilt library"
        wget -q ${PREBUILT_LINK}/sodium/libsodium_${ARCH}.zip
        unzip -qq libsodium_${ARCH}.zip
    fi

    if [ ! -d "libzmq_${ARCH}" ]; then
        echo "retrieving libzmq prebuilt library"
        wget -q ${PREBUILT_LINK}/zmq/libzmq_${ARCH}.zip
        unzip -qq libzmq_${ARCH}.zip
    fi
}

generate_flags(){
    if [ -z $1 ]; then
        echo "please provide the arch e.g arm, x86 or arm64"
        exit 1
    fi
    if [ $1 == "arm" ]; then
        export ARCH="arm"
        export TRIPLET="arm-linux-androideabi"
        export PLATFORM="16"
        export ABI="armeabi-v7a"
    fi

    if [ $1 == "x86" ]; then
        export ARCH="x86"
        export TRIPLET="i686-linux-android"
        export PLATFORM="16"
        export ABI="x86"
    fi

    if [ $1 == "arm64" ]; then
        export ARCH="arm64"
        export TRIPLET="aarch64-linux-android"
        export PLATFORM="21"
        export ABI="arm64-v8a"
    fi
}

get_libindy() {

    set -xv
    [ -z ${LIBINDY_BRANCH} ] && exit 1
    [ -z ${LIBINDY_VERSION} ] && exit 1

    if [ "$LIBINDY_BRANCH" = "stable" ]; then
        wget https://repo.sovrin.org/android/libindy/${LIBINDY_BRANCH}/${LIBINDY_VERSION}/libindy_android_${ARCH}_${LIBINDY_VERSION}.zip
    else 
        wget https://repo.sovrin.org/android/libindy/${LIBINDY_BRANCH}/${LIBINDY_VERSION}-${LIBINDY_TAG}/libindy_android_${ARCH}_${LIBINDY_VERSION}.zip
    fi

    unzip libindy_android_${ARCH}_${LIBINDY_VERSION}.zip

}

get_libnullpay() {
    set -xv
    [ -z ${LIBNULLPAY_BRANCH} ] && exit 1
    [ -z ${LIBNULLPAY_VERSION} ] && exit 1

    if [ "$LIBINDY_BRANCH" = "stable" ]; then
        wget https://repo.sovrin.org/android/libnullpay/${LIBNULLPAY_BRANCH}/${LIBNULLPAY_VERSION}/libnullpay_android_${ARCH}_${LIBNULLPAY_VERSION}.zip
    else 
        wget https://repo.sovrin.org/android/libnullpay/${LIBNULLPAY_BRANCH}/${LIBNULLPAY_VERSION}-${LIBNULLPAY_TAG}/libnullpay_android_${ARCH}_${LIBNULLPAY_VERSION}.zip
    fi

    unzip libnullpay_android_${ARCH}_${LIBNULLPAY_VERSION}.zip

}

build_vcx() {
    # For Jenkins
    LIBVCX_PATH=../vcx/libvcx/build_scripts/android/vcx/
    PREBUILT_BIN=../../../../../runtime_android_build

    # For Docker when vcx is in home dir
    #LIBVCX_PATH=~/vcx/libvcx/build_scripts/android/vcx/
    #PREBUILT_BIN=../../../../ci/scripts/runtime_android_build

    if [ ! -d libindy_${ARCH} ]; then
        echo "missing libindy_${ARCH}. Cannot proceed without it."
        exit 1
    fi
    if [ ! -d libnullpay_${ARCH} ]; then
        echo "missing libnullpay_${ARCH}. Cannot proceed without it."
        exit 1
    fi

    pushd ${LIBVCX_PATH}
    mkdir -p toolchains/
    ./build.nondocker.sh ${ARCH} ${PLATFORM} ${TRIPLET} ${PREBUILT_BIN}/openssl_${ARCH} ${PREBUILT_BIN}/libsodium_${ARCH} ${PREBUILT_BIN}/libzmq_${ARCH} ${PREBUILT_BIN}/libindy_${ARCH} ${PREBUILT_BIN}/libnullpay_${ARCH} 
    popd
    mv ${LIBVCX_PATH}libvcx_${ARCH} .

}

setup $1
get_libindy $1
get_libnullpay $1
build_vcx $1
