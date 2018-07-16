#!/usr/bin/env bash

pushd build_scripts/android
echo "Building for arm"
sh build.sh -d arm 21 arm-linux-androideabi
echo "Building for arm64"
sh build.sh -d arm64 21 aarch64-linux-android android_support
echo "Building for x86"
sh build.sh -d x86 21 i686-linux-android android_support
popd