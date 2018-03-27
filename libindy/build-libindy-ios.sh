#!/bin/sh -e

export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2l
export POD_FILE_NAME=libindy.tar.gz

echo "Build IOS POD started..."

TYPE="debug"

if [[ $# -eq 1 ]]; then
  echo "... for target $1 ..."
  cargo lipo --targets $1
else
  echo "... for all default targets ..."
  TYPE="release"
  cargo lipo --$TYPE
fi
echo 'Build completed successfully.'

WORK_DIR="out_libindy_pod"
echo "Try to create out directory: $WORK_DIR"
mkdir $WORK_DIR

if [[ ! "$WORK_DIR" || ! -d "$WORK_DIR" ]]; then
  echo "Could not create temp dir $WORK_DIR"
  exit 1
fi

echo "Packing..."

cp include/*.h $WORK_DIR
cp target/universal/$TYPE/libindy.a $WORK_DIR
cd $WORK_DIR
tar -cvzf $POD_FILE_NAME *
cd -
ls -l $WORK_DIR/$POD_FILE_NAME

echo "Packing completed."

echo "Out directory: $WORK_DIR"
