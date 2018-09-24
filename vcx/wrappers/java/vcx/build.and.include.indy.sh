#!/bin/bash

#TODO: build vcx
pushd ././../../../libvcx/build_scripts/android/vcx
./build.sh x86 21 i686-linux-android mobile_vcx openssl_x86/lib libsodium_x86/lib libzmq_x86/lib libindy/
popd
cp ././../../../libvcx/build_scripts/android/vcx/libvcx.so ./src/main/jniLibs/x86