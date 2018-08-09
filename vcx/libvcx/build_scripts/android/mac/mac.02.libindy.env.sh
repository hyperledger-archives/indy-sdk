#!/bin/sh

#3) Setup environment variables:

export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_INCREMENTAL=1
export RUST_LOG=indy=trace
export RUST_TEST_THREADS=1

#4) Setup OPENSSL_DIR variable: path to installed openssl library

for i in `ls -t /usr/local/Cellar/openssl/`; do export OPENSSL_DIR=/usr/local/Cellar/openssl/$i; break; done
#export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2n   # path changes with version number
