#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

cd $VCX_SDK/vcx/wrappers/java/vcx/

./gradlew clean assembleDebug

cd $VCX_SDK/vcx/wrappers/java/vcx/build/outputs/aar/
AAR_FILE=$(ls *-debug.aar)
AAR_VER=$(echo ${AAR_FILE} | cut -c 17-38)
# install generated .aar file
mvn install:install-file -Dfile=${AAR_FILE} -DgroupId=com.connectme \
-DartifactId=vcx -Dversion=${AAR_VER} -Dpackaging=aar
echo $AAR_VER | pbcopy

# mvn install:install-file -Dfile=${AAR_FILE} -DgroupId=com.connectme -DartifactId=vcx -Dversion=${AAR_VER} -Dpackaging=aar
