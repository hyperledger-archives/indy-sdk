#!/bin/sh

#For Evernym Mac Jenkins machines, rm the cache(~/.build_libvcx) and store libzmq-ios dir at /Users/jenkins
source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

if [ -d $WORK_DIR/libzmq-ios ]; then
    rm -rf $WORK_DIR/libzmq-ios
fi
git clone https://github.com/evernym/libzmq-ios.git $WORK_DIR/libzmq-ios
cd $WORK_DIR/libzmq-ios
git clone https://github.com/evernym/libsodium-ios.git $WORK_DIR/libzmq-ios/libsodium-ios
cd $WORK_DIR/libzmq-ios/libsodium-ios
./libsodium.rb
cd ..
./libzmq.rb

IOS_ARCHS="arm64,armv7,armv7s,i386,x86_64"
if [ ! -z "$1" ]; then
    IOS_ARCHS=$1
fi
bkpIFS="$IFS"
IFS=',()][' read -r -a archs <<<"${IOS_ARCHS}"
echo "Combining architectures: ${archs[@]}"    ##Or printf "%s\n" ${array[@]}
IFS="$bkpIFS"

cd $WORK_DIR/libzmq-ios
# Extract individual architectures for this library
for arch in ${archs[*]}
do
    mkdir -p dist/ios/lib/${arch}
    mkdir -p libsodium-ios/dist/ios/lib/${arch}
    lipo -extract ${arch} dist/ios/lib/libzmq.a -o dist/ios/lib/${arch}/libzmq-fat.a
    lipo dist/ios/lib/${arch}/libzmq-fat.a -thin $arch -output dist/ios/lib/${arch}/libzmq.a
    rm dist/ios/lib/${arch}/libzmq-fat.a
    lipo -extract ${arch} libsodium-ios/dist/ios/lib/libsodium.a -o libsodium-ios/dist/ios/lib/${arch}/libsodium-fat.a
    lipo libsodium-ios/dist/ios/lib/${arch}/libsodium-fat.a -thin $arch -output libsodium-ios/dist/ios/lib/${arch}/libsodium.a
    rm libsodium-ios/dist/ios/lib/${arch}/libsodium-fat.a
done
