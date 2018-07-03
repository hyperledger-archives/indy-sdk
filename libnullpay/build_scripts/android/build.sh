#!/bin/bash

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



if [ ! -f "android-ndk-r16b-linux-x86_64.zip" ] ; then
    echo "Downloading android-ndk-r16b-linux-x86_64.zip"
    wget -q https://dl.google.com/android/repository/android-ndk-r16b-linux-x86_64.zip
else
    echo "Skipping download android-ndk-r16b-linux-x86_64.zip"
fi

LIBNULLPAY_SRC=${WORKDIR}/indy-sdk/libnullpay
mkdir -p $LIBNULLPAY_SRC
cp -rf ./../../../libnullpay/build.rs ${LIBNULLPAY_SRC}
cp -rf ./../../../libnullpay/src ${LIBNULLPAY_SRC}
cp -rf ./../../../libnullpay/include ${LIBNULLPAY_SRC}
cp -rf ./../../../libnullpay/Cargo.toml ${LIBNULLPAY_SRC}

LIBNULLPAY_BUILDS=${WORKDIR}/libnullpay_${TARGET_ARCH}
mkdir -p ${LIBNULLPAY_BUILDS}


docker build -t libnullpay-android:latest -f android-build-env.dockerfile . --build-arg target_arch=${TARGET_ARCH} --build-arg target_api=${TARGET_API} --build-arg cross_compile=${CROSS_COMPILE} --build-arg indy_dir=${INDY_DIR} --build-arg final=${FINAL} &&
docker run libnullpay-android:latest && \
docker_id=$(docker ps -a | grep libnullpay-android:latest | grep Exited | tail -n 1 | cut -d ' ' -f 1) && \
docker_image_id=$(docker image ls | grep libnullpay-android | perl -pe 's/\s+/ /g' | cut -d ' ' -f 3) && \
docker cp ${docker_id}:/home/indy_user/libnullpay.so . && \
docker cp ${docker_id}:/home/indy_user/libnullpay.a . && \
mv libnullpay.so ${LIBNULLPAY_BUILDS}/
mv libnullpay.a ${LIBNULLPAY_BUILDS}/
echo "Libnullpay android binaries for architecture ${TARGET_ARCH} are available in ${LIBNULLPAY_BUILDS}"
docker rm ${docker_id} > /dev/null && \
docker rmi ${docker_image_id} > /dev/null
