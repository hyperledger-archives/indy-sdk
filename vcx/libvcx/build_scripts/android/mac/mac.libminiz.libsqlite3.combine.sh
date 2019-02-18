#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

INDY_SDK=$WORK_DIR/vcx-indy-sdk
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

if [ -d $WORK_DIR/combine-libs ]; then
    rm -rf $WORK_DIR/combine-libs
fi
mkdir -p $WORK_DIR/combine-libs

mkdir -p $WORK_DIR/combine-libs/libsqlite3
libcnt=0
liblist=""
for i in `find $INDY_SDK/libindy -name libsqlite3.a|grep -i release`
do
    cp $i $WORK_DIR/combine-libs/libsqlite3/$libcnt.a
    liblist="$WORK_DIR/combine-libs/libsqlite3/$libcnt.a $liblist"
    libcnt=$((libcnt + 1))
    #cp $INDY_SDK/libindy/target/aarch64-apple-ios/release/build/libsqlcipher-sys-b14af6739f126938/out/libsqlite3.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
done
lipo -create $liblist -output $WORK_DIR/combine-libs/libsqlite3/libsqlite3.a

mkdir -p $WORK_DIR/combine-libs/libminiz
libcnt=0
liblist=""
for i in `find $VCX_SDK/vcx/libvcx -name libminiz.a|grep -i release`
do
    cp $i $WORK_DIR/combine-libs/libminiz/$libcnt.a
    liblist="$WORK_DIR/combine-libs/libminiz/$libcnt.a $liblist"
    libcnt=$((libcnt + 1))
    #cp $VCX_SDK/vcx/libvcx/target/aarch64-apple-ios/release/build/miniz-sys-e7743d50325f4fdf/out/libminiz.a $VCX_SDK/vcx/wrappers/ios/ios-demo-vcx/lib
done
lipo -create $liblist -output $WORK_DIR/combine-libs/libminiz/libminiz.a
