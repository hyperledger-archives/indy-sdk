#!/bin/bash

source ./shared.functions.sh

START_DIR=$PWD
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")
WORK_DIR=$VCX_SDK/.macosbuild
WORK_DIR=$(abspath "$WORK_DIR")

IOS_TARGETS="i386-apple-ios,x86_64-apple-ios"
IOS_ARCHS="i386,x86_64"
./mac.03.libindy.build.sh debuginfo "${IOS_TARGETS}" noclean > ./mac.norms.local.rust.dev.sh.out 2>&1
./mac.06.libvcx.build.sh debuginfo "${IOS_TARGETS}" noclean >> ./mac.norms.local.rust.dev.sh.out 2>&1
./mac.11.copy.static.libs.to.app.sh >> ./mac.norms.local.rust.dev.sh.out 2>&1
./mac.12.combine.static.libs.sh libvcxall delete debuginfo "${IOS_ARCHS}" > /dev/null 2>&1
./mac.upload.ios.build.files.sh "${IOS_ARCHS}" >> ./mac.norms.local.rust.dev.sh.out 2>&1
