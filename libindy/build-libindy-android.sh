#!/usr/bin/env bash

echo "Building for arm"
sh android.build.sh -d arm
echo "Building for arm64"
sh android.build.sh -d arm64
echo "Building for x86"
sh android.build.sh -d x86