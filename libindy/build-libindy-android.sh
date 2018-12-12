#!/usr/bin/env bash

echo "Building for arm"
bash android.build.sh -d arm
echo "Building for arm64"
bash android.build.sh -d arm64
echo "Building for x86"
bash android.build.sh -d x86
