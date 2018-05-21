#!/usr/bin/env bash

cd ././../../../../
DEPS_FOLDER=indy-sdk/libindy/build_scripts/android/dependencies
LIBZMQ=${DEPS_FOLDER}/zmq/libzmq_arm
OPENSSL=${DEPS_FOLDER}/openssl/openssl_arm
LIBSODIUM=${DEPS_FOLDER}/sodium/libsodium_arm

./indy-sdk/libindy/build_scripts/android/build.sh -f arm 21 arm-linux-androideabi android_support ${OPENSSL} ${LIBSODIUM} ${LIBZMQ} && rm android-ndk-r16b-linux-x86_64.zip
