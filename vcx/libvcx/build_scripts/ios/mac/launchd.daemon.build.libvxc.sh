#!/bin/bash
# ./launchd.daemon.build.libvxc.sh > launchd.daemon.build.libvxc.sh.out 2>&1 &

START_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
echo "START_DIR: ${START_DIR}"
cd $START_DIR
cd ../../../../..
echo "VCX_DIR: $(pwd)"

GIT_BRANCH="none"
if [ ! -z "$1" ]; then
    GIT_BRANCH=$1
fi

if [ "${GIT_BRANCH}" != "none" ]; then
    git checkout . > ./vcx/libvcx/build_scripts/ios/mac/git.pull.out 2>&1
    git checkout ${GIT_BRANCH} >> ./vcx/libvcx/build_scripts/ios/mac/git.pull.out 2>&1
    git clean -f >> ./vcx/libvcx/build_scripts/ios/mac/git.pull.out 2>&1
    git clean -fd >> ./vcx/libvcx/build_scripts/ios/mac/git.pull.out 2>&1
    git pull >> ./vcx/libvcx/build_scripts/ios/mac/git.pull.out 2>&1
fi

# git checkout libindy_refactor
# git pull
# git checkout mobile_vcx
# git merge libindy_refactor
IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
IOS_ARCHS="arm64,armv7,armv7s,i386,x86_64"
#IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,x86_64-apple-ios"
#IOS_ARCHS="arm64,armv7,armv7s,x86_64"
#IOS_TARGETS="i386-apple-ios,x86_64-apple-ios"
#IOS_ARCHS="i386,x86_64"
cd vcx/libvcx/build_scripts/ios/mac
./mac.03.libindy.build.sh nodebug "${IOS_TARGETS}" cleanbuild > ./mac.03.libindy.build.sh.out 2>&1
./mac.04.libvcx.setup.sh > ./mac.04.libvcx.setup.sh.out 2>&1
./mac.06.libvcx.build.sh nodebug "${IOS_TARGETS}" cleanbuild > ./mac.06.libvcx.build.sh.out 2>&1
./mac.11.copy.static.libs.to.app.sh > ./mac.11.copy.static.libs.to.app.sh.out 2>&1
./mac.12.combine.static.libs.sh libvcxall delete nodebug "${IOS_ARCHS}" > ./mac.12.combine.static.libs.sh.out 2>&1
./mac.13.build.cocoapod.sh libvcxall "${IOS_ARCHS}" > ./mac.13.build.cocoapod.sh.out 2>&1
