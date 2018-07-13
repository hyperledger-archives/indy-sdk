#!/bin/bash

export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export BASE_DIR="../../../../.."
export WRAPPER_LIBS="vcx/wrappers/ios/vcx/lib"
IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
#IOS_TARGETS="x86_64-apple-ios"
IOS_ARCHS="armv7,armv7s,arm64,i386,x86_64"
#IOS_ARCHS="x86_64"

ls
cd ${SCRIPTS_PATH}
./mac.02.libindy.env.sh
./mac.03.libindy.build.sh nodebug "${IOS_TARGETS}"
./mac.04.libvcx.setup.sh
source ./mac.05.libvcx.env.sh
./mac.06.libvcx.build.sh nodebug "${IOS_TARGETS}"
cp -rf ~/OpenSSL-for-iPhone ${BASE_DIR}/.macosbuild
cp -rf ~/libzmq-ios ${BASE_DIR}/.macosbuild
cp -rf ~/combine-libs ${BASE_DIR}/.macosbuild
# Package for all architectures (simulator architectures included)
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxall delete nodebug "${IOS_ARCHS}"

# Package for armv7 and arm64
IOS_ARCHS="armv7,arm64"
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxpartial delete nodebug "${IOS_ARCHS}" 

# clear previous builds from jenkins machine
if [ ! -z "$(ls -A /Users/jenkins/IOSBuilds/libvcxpartial/)" ]; then
   echo "deleting old libvcxpartial builds"
   rm /Users/jenkins/IOSBuilds/libvcxpartial/*
fi
if [ ! -z "$(ls -A /Users/jenkins/IOSBuilds/libvcxall/)" ]; then
   echo "deleting old libvcxall builds"
   rm /Users/jenkins/IOSBuilds/libvcxall/*
fi

./mac.13.build.cocopod.sh libvcxpartial
./mac.13.build.cocopod.sh libvcxall