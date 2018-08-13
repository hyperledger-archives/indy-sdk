#!/bin/bash

source ./shared.functions.sh

START_DIR=$PWD
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")
WORK_DIR=$VCX_SDK/.macosbuild
WORK_DIR=$(abspath "$WORK_DIR")

COMBINED_LIB="libvcxall"
COCOAPOD_BUILD_FOLDER=~/IOSBuilds/${COMBINED_LIB}
# IMPORTANT NOTE: This soft link must exist -> ln -s /Users/norm/forge/work/code/evernym/podserver/public ~/IOSBuilds
# This allow ${COCOAPOD_BUILD_FOLDER} to point at the same folder as ${PODSERVER_DOCROOT}
PODSERVER_DOCROOT=/Users/norm/forge/work/code/evernym/podserver/public/${COMBINED_LIB}
CONNECTME_IOS=/Users/norm/forge/work/code/evernym/ConnectMe/ios
COCOAPOD_SPEC=/Users/norm/.cocoapods/repos/evernym-1/Specs/vcx/0.0.30/vcx.podspec

cd ${COCOAPOD_BUILD_FOLDER}
COCOAPOD=$(ls -t|head -1)
if [ -f "${COCOAPOD}" ]; then
    #rm ${PODSERVER_DOCROOT}/*.zip
    #mv "${COCOAPOD}" ${PODSERVER_DOCROOT}
    sed -i .bak "s/vcx\.${COMBINED_LIB}_[0-9]*\.[0-9]*_universal\.zip/${COCOAPOD}/" ${COCOAPOD_SPEC}
fi

#COCOAPOD=kdkdkdkd
#sed -i .bak "s/vcx\.${COMBINED_LIB}_[0-9]*\.[0-9]*_universal\.zip/${COCOAPOD}/" /Users/norm/.cocoapods/repos/evernym-1/Specs/vcx/0.0.30/vcx.podspec
#cat /Users/norm/.cocoapods/repos/evernym-1/Specs/vcx/0.0.30/vcx.podspec
#cp /Users/norm/.cocoapods/repos/evernym-1/Specs/vcx/0.0.30/vcx.podspec.bak /Users/norm/.cocoapods/repos/evernym-1/Specs/vcx/0.0.30/vcx.podspec

cd ${CONNECTME_IOS}
pod install
