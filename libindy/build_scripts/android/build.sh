#!/bin/bash

WORKDIR=${PWD}
DOWNLOAD_PREBUILTS="0"
FINAL="1"
while getopts ":d" opt; do
    case ${opt} in
        d) DOWNLOAD_PREBUILTS="1";;
        \?);;
    esac
done
shift $((OPTIND -1))

TARGET_ARCH=$1
TARGET_API=$2
CROSS_COMPILE=$3


download_and_unzip_deps(){
	rm -rf indy-android-dependencies
	rm dependencies
	git clone https://github.com/evernym/indy-android-dependencies.git
	pushd indy-android-dependencies/prebuilt/
	git checkout tags/v1.0.1
	find . -name "*.zip" | xargs -P 5 -I FILENAME sh -c 'unzip -o -d "$(dirname "FILENAME")" "FILENAME"'
	popd
	ln -s indy-android-dependencies/prebuilt dependencies
    export OPENSSL_DIR=dependencies/openssl/openssl_${TARGET_ARCH}
    export SODIUM_DIR=dependencies/sodium/libsodium_${TARGET_ARCH}
	export LIBZMQ_DIR=dependencies/zmq/libzmq_${TARGET_ARCH}
}

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi

if [ -z "${TARGET_API}" ]; then
    echo STDERR "Missing TARGET_API argument"
    echo STDERR "e.g. 21"
    exit 1
fi

if [ -z "${CROSS_COMPILE}" ]; then
    echo STDERR "Missing CROSS_COMPILE argument"
    echo STDERR "e.g. i686-linux-android"
    exit 1
fi

if [ "${DOWNLOAD_PREBUILTS}" == "1" ]; then
    download_and_unzip_deps
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




if [ ! -f "android-ndk-r16b-linux-x86_64.zip" ] ; then
    echo "Downloading android-ndk-r16b-linux-x86_64.zip"
    wget -q https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip
else
    echo "Skipping download android-ndk-r16b-linux-x86_64.zip"
fi

LIBINDY_SRC=${WORKDIR}/indy-sdk/libindy
mkdir -p $LIBINDY_SRC
cp -rf ./../../build.rs ${LIBINDY_SRC}
cp -rf ./../../src ${LIBINDY_SRC}
cp -rf ./../../include ${LIBINDY_SRC}
cp -rf ./../../Cargo.toml ${LIBINDY_SRC}

LIBINDY_BUILDS=${WORKDIR}/libindy_${TARGET_ARCH}
mkdir -p ${LIBINDY_BUILDS}

echo $OPENSSL_DIR
docker build -t libindy-android:latest -f android-build-env.dockerfile . --build-arg target_arch=${TARGET_ARCH} --build-arg target_api=${TARGET_API} --build-arg cross_compile=${CROSS_COMPILE} --build-arg openssl_dir=${OPENSSL_DIR} --build-arg sodium_dir=${SODIUM_DIR} --build-arg libzmq_dir=${LIBZMQ_DIR} --build-arg final=${FINAL} &&
docker run libindy-android:latest && \
docker_id=$(docker ps -a | grep libindy-android:latest | grep Exited | tail -n 1 | cut -d ' ' -f 1) && \
docker_image_id=$(docker image ls | grep libindy-android | perl -pe 's/\s+/ /g' | cut -d ' ' -f 3) && \
docker cp ${docker_id}:/home/indy_user/libindy.so . && \
docker cp ${docker_id}:/home/indy_user/libindy.a . && \
mv libindy.so ${LIBINDY_BUILDS}/
mv libindy.a ${LIBINDY_BUILDS}/
echo "Libindy android binaries for architecture ${TARGET_ARCH} are available in ${LIBINDY_BUILDS}"
docker rm ${docker_id} > /dev/null && \
docker rmi ${docker_image_id} > /dev/null
