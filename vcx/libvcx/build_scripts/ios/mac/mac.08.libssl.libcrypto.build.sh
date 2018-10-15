#!/bin/sh

#For Evernym Mac Jenkins machines, rm the cache(~/.build_libvcx) and store OpenSSL-for-iPhone dir at /Users/jenkins
source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

if [ -d $WORK_DIR/OpenSSL-for-iPhone ]; then
    rm -rf $WORK_DIR/OpenSSL-for-iPhone.bak
    mv $WORK_DIR/OpenSSL-for-iPhone $WORK_DIR/OpenSSL-for-iPhone.bak
fi
git clone https://github.com/x2on/OpenSSL-for-iPhone.git $WORK_DIR/OpenSSL-for-iPhone
cd $WORK_DIR/OpenSSL-for-iPhone

# for i in `ls -t /usr/local/Cellar/openssl/`; do export OPENSSL_VER=$i; break; done
# OPENSSL_VER=`echo $OPENSSL_VER|awk '{split($1,A,"_"); print A[1]}'`
OPENSSL_VER="1.0.2o"
echo "Using version number: $OPENSSL_VER"
./build-libssl.sh --version=$OPENSSL_VER

IOS_ARCHS="arm64,armv7,armv7s,i386,x86_64"
if [ ! -z "$1" ]; then
    IOS_ARCHS=$1
fi
bkpIFS="$IFS"
IFS=',()][' read -r -a archs <<<"${IOS_ARCHS}"
echo "Combining architectures: ${archs[@]}"    ##Or printf "%s\n" ${array[@]}
IFS="$bkpIFS"

cd $WORK_DIR/OpenSSL-for-iPhone/lib
# Extract individual architectures for this library
for arch in ${archs[*]}
do
    mkdir -p ${arch}
    lipo -extract ${arch} libssl.a -o ${arch}/libssl-fat.a
    lipo ${arch}/libssl-fat.a -thin $arch -output ${arch}/libssl.a
    rm ${arch}/libssl-fat.a
    lipo -extract ${arch} libcrypto.a -o ${arch}/libcrypto-fat.a
    lipo ${arch}/libcrypto-fat.a -thin $arch -output ${arch}/libcrypto.a
    rm ${arch}/libcrypto-fat.a
done
