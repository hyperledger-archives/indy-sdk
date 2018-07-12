#!/bin/sh

source ./mac.05.libvcx.env.sh
cd ../../..
DEBUG_SYMBOLS="debuginfo"

if [ ! -z "$1" ]; then
    DEBUG_SYMBOLS=$1
fi

if [ "$DEBUG_SYMBOLS" = "nodebug" ]; then
    sed -i .bak 's/debug = true/debug = false/' Cargo.toml
fi

cargo clean
cargo update
# To build for macos
#cargo build
#export LIBINDY_DIR=/usr/local/lib
#export RUST_BACKTRACE=1
# To build for iOS
#LIBINDY_DIR=/usr/local/lib RUST_BACKTRACE=1 cargo lipo --release
cargo lipo --release --verbose --targets="aarch64-apple-ios,armv7-apple-ios,armv7s-apple-ios,i386-apple-ios,x86_64-apple-ios"
#LIBINDY_DIR=/usr/local/lib RUST_BACKTRACE=1 cargo lipo
#LIBINDY_DIR=/usr/local/lib cargo test

#cargo test

#lipo -info 