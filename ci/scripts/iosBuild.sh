#!/bin/bash

export SCRIPTS_PATH="vcx/libvcx/build_scripts/ios/mac"
export BASE_DIR="../../../../.."
export WRAPPER_LIBS="vcx/wrappers/ios/vcx/lib"


ls
cd ${SCRIPTS_PATH}
./mac.02.libindy.env.sh
./mac.03.libindy.build.sh
./mac.04.libvcx.setup.sh
source ./mac.05.libvcx.env.sh
./mac.06.libvcx.build.sh nodebug
cp -rf ~/OpenSSL-for-iPhone ${BASE_DIR}/.macosbuild
cp -rf ~/libzmq-ios ${BASE_DIR}/.macosbuild
cp -rf ~/combine-libs ${BASE_DIR}/.macosbuild
# Package for all architectures (simulator architectures included)
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxall delete nodebug

# Package for armv7 and arm64
./mac.11.copy.static.libs.to.app.sh
./mac.12.combine.static.libs.sh libvcxpartial delete nodebug

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