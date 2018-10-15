#!/bin/bash

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

OPENSSL_DIR=$WORK_DIR/openssl_for_ios_and_android
INDY_SDK=$WORK_DIR/vcx-indy-sdk
VCX_SDK=$START_DIR/../../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

declare -a archs=(
    "x86" "x86" "i686-linux-android" "x86"
    )
archslen=${#archs[@]}

for (( arch=0; arch<${archslen}; arch=arch+4 ));
do
    export ndk_arch=${archs[$arch]}
    export target_arch=${archs[$arch+1]}
    export cross_compile=${archs[$arch+2]}
    export openssl_arch=${archs[$arch+3]}

    mkdir -p $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    ln -f -v $OPENSSL_DIR/output/android/openssl-${openssl_arch}/lib/libssl.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    ln -f -v $OPENSSL_DIR/output/android/openssl-${openssl_arch}/lib/libcrypto.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    # ln -f -v $INDY_SDK/libindy/target/${cross_compile}/release/libindy.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    # ln -f -v $INDY_SDK/libindy/target/${cross_compile}/debug/libindy.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    ln -f -v $VCX_SDK/vcx/libvcx/target/${cross_compile}/debug/libvcx.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    ln -f -v $WORK_DIR/libzmq-android/libsodium/libsodium_${target_arch}/lib/libsodium.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    ln -f -v $WORK_DIR/libzmq-android/zmq/libzmq_${target_arch}/lib/libzmq.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    ln -f -v $WORK_DIR/libz-android/zlib/lib/${target_arch}/libz.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
    ln -f -v $WORK_DIR/libsqlite3-android/sqlite3-android/obj/local/${openssl_arch}/libsqlite3.a $VCX_SDK/vcx/wrappers/java/vcx/src/main/jniLibs/${target_arch}
done