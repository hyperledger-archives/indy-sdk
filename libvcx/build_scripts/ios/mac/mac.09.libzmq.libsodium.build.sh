#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

if [ -d $WORK_DIR/libzmq-ios ]; then
    rm -rf $WORK_DIR/libzmq-ios
fi
git clone https://github.com/azawawi/libzmq-ios.git $WORK_DIR/libzmq-ios
cd $WORK_DIR/libzmq-ios
git clone https://github.com/azawawi/libsodium-ios.git $WORK_DIR/libzmq-ios/libsodium-ios
cd $WORK_DIR/libzmq-ios/libsodium-ios
./libsodium.rb
cd ..
./libzmq.rb
