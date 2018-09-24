#!/usr/bin/env bash

export INDY_PREFIX_DIR="libindy"
echo "Building for arm"
sh android.build.sh -d arm "${INDY_PREFIX_DIR}/libindy_arm"
echo "Building for arm64"
sh android.build.sh -d arm64 "${INDY_PREFIX_DIR}/libindy_arm64"
echo "Building for x86"
sh android.build.sh -d x86 "${INDY_PREFIX_DIR}/libindy_x86"