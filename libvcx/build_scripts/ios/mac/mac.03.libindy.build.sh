#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")
SHA_HASH_DIR=$START_DIR/../..
SHA_HASH_DIR=$(abspath "$SHA_HASH_DIR")

source ./mac.02.libindy.env.sh

CLEAN_BUILD="cleanbuild"
if [ ! -z "$3" ]; then
    CLEAN_BUILD=$3
fi

if [ -d $WORK_DIR/vcx-indy-sdk ]; then
    #rm -rf $WORK_DIR/vcx-indy-sdk
    cd $WORK_DIR/vcx-indy-sdk
else
    git clone https://github.com/hyperledger/indy-sdk.git $WORK_DIR/vcx-indy-sdk
    cd $WORK_DIR/vcx-indy-sdk
fi

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    git checkout .
    git checkout master
    git clean -f
    git clean -fd
    git pull
    git checkout `cat $SHA_HASH_DIR/libindy.commit.sha1.hash.txt`
    #cd $WORK_DIR/vcx-indy-sdk
    #git checkout tags/v1.3.0
else
    git checkout -- libindy/Cargo.toml
    git checkout -- libnullpay/Cargo.toml
fi

git log -1 > $WORK_DIR/hyperledger.indy-sdk.git.commit.log

DEBUG_SYMBOLS="debuginfo"
if [ ! -z "$1" ]; then
    DEBUG_SYMBOLS=$1
fi

IOS_TARGETS="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
if [ ! -z "$2" ]; then
    IOS_TARGETS=$2
fi

#########################################################################################################################
# Now build libindy
#########################################################################################################################
cd $WORK_DIR/vcx-indy-sdk/libindy

#if [ "$DEBUG_SYMBOLS" = "debuginfo" ]; then
    cat $START_DIR/cargo.toml.add.debug.txt >> Cargo.toml
#fi
if [ "$DEBUG_SYMBOLS" = "nodebug" ]; then
    sed -i .bak 's/debug = true/debug = false/' Cargo.toml
fi

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    cargo clean
    # cargo update
fi

# To build for macos
#cargo build
# To build for iOS
#echo "cargo lipo --release --verbose --targets=${IOS_TARGETS}"
# cargo lipo --release --verbose --targets="${IOS_TARGETS}"
cargo lipo --release --targets="${IOS_TARGETS}"
#cargo lipo

#########################################################################################################################
# Now build libnullpay
#########################################################################################################################
cd $WORK_DIR/vcx-indy-sdk/libnullpay

# if [ "$DEBUG_SYMBOLS" = "debuginfo" ]; then
#     cat $START_DIR/cargo.toml.add.debug.txt >> Cargo.toml
# fi
if [ "$DEBUG_SYMBOLS" = "nodebug" ]; then
    sed -i .bak 's/debug = true/debug = false/' Cargo.toml
fi

if [ "$CLEAN_BUILD" = "cleanbuild" ]; then
    cargo clean
    # cargo update
fi

# Replace '\"cdylib\"' with '\"staticlib\", \"cdylib\"' in Cargo.toml
#sed -i .bak 's/\"cdylib\"/\"staticlib\", \"cdylib\"/' Cargo.toml

# To build for macos
#cargo build
# To build for iOS
#echo "cargo lipo --release --verbose --targets=${IOS_TARGETS}"
# cargo lipo --release --verbose --targets="${IOS_TARGETS}"
cargo lipo --release --targets="${IOS_TARGETS}"
#cargo lipo
