#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

cd $VCX_SDK/vcx/wrappers/java/vcx/
# sed -i .bak 's/evernym/androidbuild1/g' local.properties
./gradlew clean assembleDebug
cd $VCX_SDK/vcx/wrappers/java/vcx/build/outputs/aar/
AAR_FILE=$(ls *-debug.aar)
AAR_VER=$(echo ${AAR_FILE} | cut -c 17-38)
# install generated .aar file
mvn install:install-file -Dfile=${AAR_FILE} -DgroupId=com.connectme \
-DartifactId=vcx -Dversion=${AAR_VER} -Dpackaging=aar

# rm -rf $WORK_DIR/aar
# mkdir -p $WORK_DIR/aar
# cd $WORK_DIR/aar
# unzip $START_DIR/vcx.aar.template.aar
# cp $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/arm/*.so ./jni/arm
# cp $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/armv7/*.so ./jni/armv7
# #cp $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/armeabi-v7a/*.so ./jni/armeabi-v7a
# cp $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/x86/*.so ./jni/x86
# cp $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/arm64/*.so ./jni/arm64
# cp $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni/x86_64/*.so ./jni/x86_64
