#!/usr/bin/env bash

pushd build_scripts/android
echo "Building for arm"
sh build.sh arm 21 arm-linux-androideabi libindy_arm
echo "Building for arm64"
sh build.sh arm64 21 aarch64-linux-android libindy_arm64
echo "Building for x86"
sh build.sh x86 21 i686-linux-android libindy_x86
popd