#!/usr/bin/env bash

cd ././../../../../
DEPS_FOLDER=indy-sdk/libindy/build_scripts/android/dependencies
LIBZMQ=${DEPS_FOLDER}/zmq/libzmq_arm64
OPENSSL=${DEPS_FOLDER}/openssl/openssl_arm64
LIBSODIUM=${DEPS_FOLDER}/sodium/libsodium_arm64
./indy-sdk/libindy/build_scripts/android/build.sh -f arm64 21 aarch64-linux-android android_support ${OPENSSL} ${LIBSODIUM} ${LIBZMQ}