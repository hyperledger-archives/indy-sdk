#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

COMBINED_LIB=$1

DATETIME=$(date +"%Y%m%d.%H%M")

cd $VCX_SDK/vcx/wrappers/ios/vcx
mv lib/libvcx.a lib/libvcx.a.original
cp -v lib/${COMBINED_LIB}.a lib/libvcx.a
xcodebuild -project vcx.xcodeproj -scheme vcx -configuration Release CONFIGURATION_BUILD_DIR=. clean > $START_DIR/xcodebuild.vcx.framework.build.out 2>&1
if [ ${COMBINED_LIB} = "libvcxall" ]; then
    xcodebuild -project vcx.xcodeproj -scheme vcx -configuration Release -sdk iphonesimulator CONFIGURATION_BUILD_DIR=. build >> $START_DIR/xcodebuild.vcx.framework.build.out 2>&1
    mv vcx.framework vcx.framework.iphonesimulator
fi 
xcodebuild -project vcx.xcodeproj -scheme vcx -configuration -sdk iphoneos CONFIGURATION_BUILD_DIR=. build >> $START_DIR/xcodebuild.vcx.framework.build.out 2>&1

mv lib/libvcx.a.original lib/libvcx.a
if [ ${COMBINED_LIB} = "libvcxall" ]; then
    lipo -create -output combined.ios.vcx vcx.framework/vcx vcx.framework.iphonesimulator/vcx
    rm -rf vcx.framework.iphonesimulator
fi 

mkdir -p vcx.framework/lib
cp -v lib/${COMBINED_LIB}.a vcx.framework/lib/libvcx.a
mkdir -p vcx.framework/Headers
cp -v ConnectMeVcx.h vcx.framework/Headers
cp -v include/libvcx.h vcx.framework/Headers
cp -v vcx/vcx.h vcx.framework/Headers
if [ -d $VCX_SDK/vcx/wrappers/ios/vcx/tmp ]; then
    rm -rf $VCX_SDK/vcx/wrappers/ios/vcx/tmp
fi
mkdir -p $VCX_SDK/vcx/wrappers/ios/vcx/tmp/vcx/
cp -rvp vcx.framework $VCX_SDK/vcx/wrappers/ios/vcx/tmp/vcx/
cd $VCX_SDK/vcx/wrappers/ios/vcx/tmp
cp $WORK_DIR/evernym.vcx-sdk.git.commit.log $VCX_SDK/vcx/wrappers/ios/vcx/tmp/vcx/
cp $WORK_DIR/hyperledger.indy-sdk.git.commit.log $VCX_SDK/vcx/wrappers/ios/vcx/tmp/vcx/

zip -r vcx.${COMBINED_LIB}_${DATETIME}_universal.zip vcx

cp $VCX_SDK/vcx/wrappers/ios/vcx/tmp/vcx.${COMBINED_LIB}_${DATETIME}_universal.zip ~/IOSBuilds/${COMBINED_LIB}

