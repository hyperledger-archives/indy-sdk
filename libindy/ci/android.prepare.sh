#!/bin/bash
set -e
export TARGET_ARCH=$2

export ANDROID_BUILD_FOLDER="/tmp/android_build"
source setup.android.env.sh

echo ">> in runner script"

download_sdk
download_and_setup_toolchain
download_emulator