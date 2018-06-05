#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

DATETIME=$1
if [ "$DATETIME" = "" ]; then
    echo "You must pass the datetime as the first parameter to the script. (i.e. 20180522.1354 - YYYYmmdd.hhMM)"
    exit 1
fi

cd $VCX_SDK/vcx/wrappers/java/android/vcxtest/app/jni

mv libvcxall_arm.zip libvcxall_${DATETIME}_arm.zip
curl --insecure -u normjarvis -X POST -F file=@./libvcxall_${DATETIME}_arm.zip https://kraken.corp.evernym.com/repo/android/upload
sudo cp -v ./libvcxall_${DATETIME}_arm.zip  /usr/local/var/www/download/android

mv libvcxall_arm64.zip libvcxall_${DATETIME}_arm64.zip
curl --insecure -u normjarvis -X POST -F file=@./libvcxall_${DATETIME}_arm64.zip https://kraken.corp.evernym.com/repo/android/upload
sudo cp -v ./libvcxall_${DATETIME}_arm64.zip  /usr/local/var/www/download/android

mv libvcxall_armv7.zip libvcxall_${DATETIME}_armv7.zip
curl --insecure -u normjarvis -X POST -F file=@./libvcxall_${DATETIME}_armv7.zip https://kraken.corp.evernym.com/repo/android/upload
sudo cp -v ./libvcxall_${DATETIME}_armv7.zip  /usr/local/var/www/download/android

mv libvcxall_x86.zip libvcxall_${DATETIME}_x86.zip
curl --insecure -u normjarvis -X POST -F file=@./libvcxall_${DATETIME}_x86.zip https://kraken.corp.evernym.com/repo/android/upload
sudo cp -v ./libvcxall_${DATETIME}_x86.zip  /usr/local/var/www/download/android

mv libvcxall_x86_64.zip libvcxall_${DATETIME}_x86-64.zip
curl --insecure -u normjarvis -X POST -F file=@./libvcxall_${DATETIME}_x86-64.zip https://kraken.corp.evernym.com/repo/android/upload
sudo cp -v ./libvcxall_${DATETIME}_x86-64.zip  /usr/local/var/www/download/android
