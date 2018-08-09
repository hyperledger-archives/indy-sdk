#!/bin/bash

setup() {
    echo "Working Directory: ${PWD}"
    set -e
    export ARCH=$1

    export PATH=$PATH:/opt/gradle/gradle-3.4.1/bin
    export PATH=${PATH}:$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools:$ANDROID_HOME/build-tools/25.0.2/
    export PATH=${HOME}/.cargo/bin:${PATH}
    export EVERNYM_REPO=https://repo.corp.evernym.com/filely/android
    export VCX_BASE=../vcx
    # For docker
    # export VCX_BASE=${HOME}/vcx

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

    ANDROID_JNI_LIB=${VCX_BASE}/wrappers/java/vcx/src/main/jniLibs

    mkdir -p ${ANDROID_JNI_LIB}/arm
    mkdir -p ${ANDROID_JNI_LIB}/x86
    mkdir -p ${ANDROID_JNI_LIB}/arm64
}


retrieve_prebuilt_binaries() {
    OPENSSL_PREBUILT_ZIP=openssl_1.1.0h-201807251800_r17b.zip
    SODIUM_PREBUILT_ZIP=sodium_1.0.14-201807251900_r17b.zip
    ZMQ_PREBUILT_ZIP=zmq_4.2.5-201807252000_r17b.zip

    if [ ! -d "openssl_prebuilt" ]; then
        echo "retrieving openssl prebuilt library"
        wget -q ${EVERNYM_REPO}/${OPENSSL_PREBUILT_ZIP}
        unzip -qq ${OPENSSL_PREBUILT_ZIP}
    fi
    if [ ! -d "sodium_prebuilt" ]; then
        echo "retrieving openssl prebuilt library"
        wget -q ${EVERNYM_REPO}/${SODIUM_PREBUILT_ZIP}
        unzip -qq ${SODIUM_PREBUILT_ZIP}
    fi
    if [ ! -d "zmq_prebuilt" ]; then
        echo "retrieving openssl prebuilt library"
        wget -q ${EVERNYM_REPO}/${ZMQ_PREBUILT_ZIP}
        unzip -qq ${ZMQ_PREBUILT_ZIP}
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

    if [ ! -d "libindy_${ARCH}" ]; then
        if [ "$LIBINDY_BRANCH" = "stable" ]; then
            wget https://repo.sovrin.org/android/libindy/${LIBINDY_BRANCH}/${LIBINDY_VERSION}/libindy_android_${ARCH}_${LIBINDY_VERSION}.zip
        else 
            wget https://repo.sovrin.org/android/libindy/${LIBINDY_BRANCH}/${LIBINDY_VERSION}-${LIBINDY_TAG}/libindy_android_${ARCH}_${LIBINDY_VERSION}.zip
        fi

        unzip libindy_android_${ARCH}_${LIBINDY_VERSION}.zip
    fi
}

get_libsovtoken() {
    set -xv
    # Todo: This artifact was manually uploaded to this repo. Eventually, the file format will change. That is why it is hardcoded
    LIBSOVTOKEN_ZIP=libsovtoken_0.8.1-201807262112-cbb1520_all.zip
    if [ ! -d "libsovtoken" ]; then
        echo "retrieving libsovtoken prebuilt library"
        wget ${EVERNYM_REPO}/${LIBSOVTOKEN_ZIP}
        unzip ${LIBSOVTOKEN_ZIP}
    fi
}

build_vcx() {
    # For Jenkins
    LIBVCX_PATH=${VCX_BASE}/libvcx/build_scripts/android/vcx/
    PREBUILT_BIN=../../../../../runtime_android_build
    # For Docker when vcx is in home dir
    #PREBUILT_BIN=../../../../ci/scripts/runtime_android_build
    # PREBUILT_BIN=$(realpath ${VCX_BASE}/ci/scripts/runtime_android_build)

    if [ ! -d libindy_${ARCH} ]; then
        echo "missing libindy_${ARCH}. Cannot proceed without it."
        exit 1
    fi
    if [ ! -d libsovtoken ]; then
        echo "missing libsovtoken. Cannot proceed without it."
        exit 1
    fi

    pushd ${LIBVCX_PATH}
    mkdir -p toolchains/
    ./build.nondocker.sh ${ARCH} ${PLATFORM} ${TRIPLET} ${PREBUILT_BIN}/openssl_prebuilt/${ARCH} ${PREBUILT_BIN}/sodium_prebuilt/${ARCH} ${PREBUILT_BIN}/zmq_prebuilt/${ARCH} ${PREBUILT_BIN}/libindy_${ARCH} ${PREBUILT_BIN}/libsovtoken/${TRIPLET} 
    popd
    mv ${LIBVCX_PATH}libvcx_${ARCH} .

}

setup $1
get_libindy $1
get_libsovtoken
build_vcx $1
