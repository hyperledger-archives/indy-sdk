#!/bin/sh

source ./mac.05.libvcx.env.sh
cd ../..
cargo clean
# To build for macos
#cargo build
#export LIBINDY_DIR=/usr/local/lib
#export RUST_BACKTRACE=1
# To build for iOS
#LIBINDY_DIR=/usr/local/lib RUST_BACKTRACE=1 cargo lipo --release
cargo lipo --release --verbose
#LIBINDY_DIR=/usr/local/lib RUST_BACKTRACE=1 cargo lipo
#LIBINDY_DIR=/usr/local/lib cargo test
cargo test
