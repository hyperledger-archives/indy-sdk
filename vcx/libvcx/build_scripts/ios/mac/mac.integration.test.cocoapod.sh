#!/bin/bash

source ./shared.functions.sh

START_DIR=$PWD
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")
WORK_DIR=$VCX_SDK/.macosbuild
WORK_DIR=$(abspath "$WORK_DIR")

TEST_WORKSPACE_ROOT=/Users/norm/forge/work/code/evernym/testlibvcx
TEST_WORKSPACE=testlibvcx.xcworkspace
TEST_SCHEME=testlibvcxTests

cd $TEST_WORKSPACE_ROOT
pod install

xcodebuild -workspace ${TEST_WORKSPACE} -scheme ${TEST_SCHEME} -sdk iphonesimulator build-for-testing
## Need to do: brew install xctool
xctool -workspace ${TEST_WORKSPACE} -scheme ${TEST_SCHEME} run-tests -sdk iphonesimulator