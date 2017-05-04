#!/bin/sh 

export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2k
export EVERNYM_REPO_KEY=~/Documents/EvernymRepo

echo "\nBuild IOS POD started..."
#cargo lipo
echo 'Build completed successfully.'

WORK_DIR=`mktemp -d`

echo "Try to create temporary directory: $WORK_DIR"

if [[ ! "$WORK_DIR" || ! -d "$WORK_DIR" ]]; then
  echo "Could not create temp dir $WORK_DIR"
  exit 1
fi

echo "Packing...\n\n"

cp include/*.h $WORK_DIR
cp target/universal/debug/libsovrin.a $WORK_DIR
CUR_DIR=`pwd`
cd $WORK_DIR
tar -cvzf libsovrin-ios.tar.gz *

echo "\nPacking completed."
cd $CUR_DIR

echo "Uploading...."

cat <<EOF | sftp -i $EVERNYM_REPO_KEY repo@54.187.56.182
ls -l
help
EOF


echo "Cleanup temporary directory: $WORK_DIR"
rm -rf "$WORK_DIR"

