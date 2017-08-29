#!/bin/sh 

export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2l
export EVERNYM_REPO_KEY=~/Documents/EvernymRepo
export LIBINDY_POD_VERSION=0.0.3
export POD_FILE_NAME=libindy-core-ios.tar.gz

echo "\nBuild IOS POD started..."
cargo lipo
echo 'Build completed successfully.'

WORK_DIR=`mktemp -d`

echo "Try to create temporary directory: $WORK_DIR"

if [[ ! "$WORK_DIR" || ! -d "$WORK_DIR" ]]; then
  echo "Could not create temp dir $WORK_DIR"
  exit 1
fi

echo "Packing...\n\n"

cp include/*.h $WORK_DIR
cp target/universal/debug/libindy.a $WORK_DIR
CUR_DIR=`pwd`
cd $WORK_DIR
tar -cvzf $POD_FILE_NAME *
ls -l $WORK_DIR/$POD_FILE_NAME

echo "\nPacking completed."
cd $CUR_DIR

#echo "Uploading...."

#cat <<EOF | sftp -i $EVERNYM_REPO_KEY repo@54.187.56.182
#ls -l /var/repositories/libindy/ios/stable/libindy-core/$LIBINDY_POD_VERSION/$POD_FILE_NAME
#rm /var/repositories/libindy/ios/stable/libindy-core/$LIBINDY_POD_VERSION/$POD_FILE_NAME
#rmdir /var/repositories/libindy/ios/stable/libindy-core/$LIBINDY_POD_VERSION
#mkdir /var/repositories/libindy/ios/stable/libindy-core/$LIBINDY_POD_VERSION
#cd /var/repositories/libindy/ios/stable/libindy-core/$LIBINDY_POD_VERSION
#put $WORK_DIR/$POD_FILE_NAME
#ls -l /var/repositories/libindy/ios/stable/libindy-core/$LIBINDY_POD_VERSION
#EOF

echo "Temporary directory: $WORK_DIR"
#rm -rf "$WORK_DIR"

