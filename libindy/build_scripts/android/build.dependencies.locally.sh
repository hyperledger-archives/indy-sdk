#!/usr/bin/env bash

WORKDIR=${PWD}
LIBZMQ_WORKDIR=dependencies/zmq
LIBSSL_WORKDIR=dependencies/openssl
LIBSODIUM_WORKDIR=dependencies/sodium

pushd $LIBSSL_WORKDIR
./build.sh
popd

pushd $LIBSODIUM_WORKDIR
./build.sh arm 21 arm-linux-androideabi
unzip libsodium_arm.zip
cp libsodium_arm.zip ${WORKDIR}/${LIBZMQ_WORKDIR}/

./build.sh arm64 21 aarch64-linux-android
unzip libsodium_arm64.zip
cp libsodium_arm64.zip ${WORKDIR}/${LIBZMQ_WORKDIR}/

./build.sh x86 21 i686-linux-android
unzip libsodium_x86.zip
cp libsodium_x86.zip ${WORKDIR}/${LIBZMQ_WORKDIR}/
popd

pushd ${LIBZMQ_WORKDIR}
unzip libsodium_arm.zip
unzip libsodium_arm64.zip
unzip libsodium_x86.zip
./build.sh arm 21 arm-linux-androideabi libsodium_arm
unzip libzmq_arm.zip
./build.sh arm64 21 aarch64-linux-android libsodium_arm64
unzip libzmq_arm64.zip
./build.sh x86 21 i686-linux-android libsodium_x86
unzip libzmq_x86.zip
popd