#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

cd $VCX_SDK/vcx/wrappers/java/vcx/

./gradlew clean assembleRelease

cd $VCX_SDK/vcx/wrappers/java/vcx/build/outputs/aar/
AAR_FILE=$(ls *-release.aar)
AAR_VER=$(echo ${AAR_FILE} | cut -c 17-38)
# install generated .aar file
mvn install:install-file -Dfile=${AAR_FILE} -DgroupId=com.connectme \
-DartifactId=vcx -Dversion=${AAR_VER} -Dpackaging=aar
echo $AAR_VER | pbcopy

# Upload to public cloud repo
mvn -e deploy:deploy-file \
   -Durl="https://evernym.mycloudrepo.io/repositories/libvcx-android" \
   -DrepositoryId="io.cloudrepo" \
   -Dversion="${AAR_VER}" \
   -Dfile=${AAR_FILE} \
   -DartifactId="vcx" \
   -Dpackaging="aar" \
   -DgroupId="com.evernym"
