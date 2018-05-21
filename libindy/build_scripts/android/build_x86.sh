#!/usr/bin/env bash

cd ././../../../../
DEPS_FOLDER=indy-sdk/libindy/build_scripts/android/dependencies
LIBZMQ=${DEPS_FOLDER}/zmq/libzmq_x86
OPENSSL=${DEPS_FOLDER}/openssl/openssl_x86
LIBSODIUM=${DEPS_FOLDER}/sodium/libsodium_x86
./indy-sdk/libindy/build_scripts/android/build.sh -f x86 21 i686-linux-android android_support ${OPENSSL} ${LIBSODIUM} ${LIBZMQ}