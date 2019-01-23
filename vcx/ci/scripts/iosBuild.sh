#!/bin/bash

set -e
export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export BASE_DIR="../../../../.."
export WRAPPER_BASE="vcx/wrappers/ios/vcx"
export WRAPPER_LIBS="vcx/wrappers/ios/vcx/lib"
IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,i386-apple-ios,x86_64-apple-ios"
IOS_ARCHS="arm64,armv7,i386,x86_64"
#IOS_TARGETS="x86_64-apple-ios,i386-apple-ios"
#IOS_ARCHS="x86_64,i386"


function printStartProgress {
		echo "=================="
		echo "Starting"
		echo $1
		echo "=================="
		echo
}

function printEndProgress {
		echo
		echo "+++++++++++++++++"
		echo "Finished"
		echo $1
		echo "+++++++++++++++++"
}

function progress {
		printStartProgress $1
		$1
		printEndProgress $1
}

cd ${SCRIPTS_PATH}

progress ./mac.01.libindy.setup.sh
progress ./mac.02.libindy.env.sh
progress ./mac.03.libindy.build.sh
#./mac.04.libvcx.setup.sh
progress 'source ./mac.05.libvcx.env.sh'
cp -rf ~/OpenSSL-for-iPhone ${BASE_DIR}/.macosbuild
cp -rf ~/libzmq-ios ${BASE_DIR}/.macosbuild
cp -rf ~/combine-libs ${BASE_DIR}/.macosbuild
./mac.06.libvcx.build.sh nodebug "${IOS_TARGETS}" cleanbuild

# clear previous builds from jenkins machine
if [ ! -z "$(ls -A /Users/jenkins/IOSBuilds/libvcxpartial/)" ]; then
   echo "deleting old libvcxpartial builds"
   rm /Users/jenkins/IOSBuilds/libvcxpartial/*
fi
if [ ! -z "$(ls -A /Users/jenkins/IOSBuilds/libvcxall/)" ]; then
   echo "deleting old libvcxall builds"
   rm /Users/jenkins/IOSBuilds/libvcxall/*
fi

# Retrieve libindy ios wrapper
#git clone https://github.com/hyperledger/indy-sdk.git
#cd indy-sdk
#git checkout tags/v1.7.0
#cd ..
#cp -rf indy-sdk/wrappers/ios/libindy-pod/Indy ${WRAPPER_BASE}
# Package for all architectures (simulator architectures included)
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxall delete nodebug "${IOS_ARCHS}"
./mac.13.build.cocoapod.sh libvcxall

# Package for armv7 and arm64
IOS_ARCHS="arm64,armv7"
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxpartial delete nodebug "${IOS_ARCHS}"
./mac.13.build.cocoapod.sh libvcxpartial "${IOS_ARCHS}"

