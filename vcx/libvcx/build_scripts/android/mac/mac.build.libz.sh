#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

mkdir -p $WORK_DIR/libz-android

# Download and build zlib for android
mkdir -p $WORK_DIR/libz-android/zlib/include
ZLIB_DIR=$WORK_DIR/libz-android/zlib-1.2.11
cd $WORK_DIR/libz-android
if [ ! -f zlib-1.2.11.tar.gz ]; then
    wget https://zlib.net/zlib-1.2.11.tar.gz
fi
if [ ! -d $ZLIB_DIR ]; then
    tar -zxf zlib-1.2.11.tar.gz
fi
cd $ZLIB_DIR
cp zconf.h $WORK_DIR/libz-android/zlib/include/ 
cp zlib.h $WORK_DIR/libz-android/zlib/include/


declare -a archs=(
    "arm" "arm" "arm-linux-androideabi"
    "arm" "armv7" "arm-linux-androideabi"
    "arm64" "arm64" "aarch64-linux-android"
    "x86" "x86" "i686-linux-android"
    "x86_64" "x86_64" "x86_64-linux-android"
    )
archslen=${#archs[@]}

for (( arch=0; arch<${archslen}; arch=arch+3 ));
do
    export ndk_arch=${archs[$arch]}
    export target_arch=${archs[$arch+1]}
    export cross_compile=${archs[$arch+2]}
    export TOOLCHAIN=$WORK_DIR/NDK/${ndk_arch}
    export PATH=$PATH:$TOOLCHAIN/bin
    export SYSROOT=$TOOLCHAIN/sysroot
    export ARCH=${target_arch}
    export CC=${cross_compile}-clang
    export CXX=${cross_compile}-clang++
    export AR=${cross_compile}-ar
    export AS=${cross_compile}-as
    export LD=${cross_compile}-ld
    export RANLIB=${cross_compile}-ranlib
    export NM=${cross_compile}-nm
    export STRIP=${cross_compile}-strip
    export CHOST=${cross_compile}
    make clean
    ./configure --shared
    make

    mkdir -p $WORK_DIR/libz-android/zlib/lib/${target_arch}
    cp libz.a $WORK_DIR/libz-android/zlib/lib/${target_arch}/libz.a
    cp libz.so.1.2.11 $WORK_DIR/libz-android/zlib/lib/${target_arch}/libz.so
done
