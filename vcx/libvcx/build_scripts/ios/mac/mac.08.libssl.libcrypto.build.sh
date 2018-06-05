#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

if [ -d $WORK_DIR/OpenSSL-for-iPhone ]; then
    rm -rf $WORK_DIR/OpenSSL-for-iPhone
fi
git clone https://github.com/x2on/OpenSSL-for-iPhone.git $WORK_DIR/OpenSSL-for-iPhone
cd $WORK_DIR/OpenSSL-for-iPhone

for i in `ls -t /usr/local/Cellar/openssl/`; do export OPENSSL_VER=$i; break; done
OPENSSL_VER=`echo $OPENSSL_VER|awk '{split($1,A,"_"); print A[1]}'`
echo "Using version number: $OPENSSL_VER"
./build-libssl.sh --version=$OPENSSL_VER
