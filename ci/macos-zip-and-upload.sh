#!/bin/bash

set -e
set -x

if [ "$1" = "--help" ] ; then
  echo "Usage: <folder> <package> <version> <key> <type> <number>"
  return
fi

folder="$1"
package="$2"
version="$3"
key="$4"
type="$5"
number="$6"

[ -z $folder ] && exit 1
[ -z $package ] && exit 2
[ -z $version ] && exit 3
[ -z $key ] && exit 4
[ -z $type ] && exit 5
[ -z $number ] && exit 6

TEMP_ARCH_DIR=./${package}-zip

mkdir -p ${TEMP_ARCH_DIR}/lib
cp -r ${folder}/include ${TEMP_ARCH_DIR}
cp ${folder}/target/release/*.a ${TEMP_ARCH_DIR}/lib/
cp ${folder}//target/release/*.dylib ${TEMP_ARCH_DIR}/lib/

cd ${TEMP_ARCH_DIR} && zip -r ${package}_${version}.zip ./* && mv ${package}_${version}.zip .. && cd ..

rm -rf ${TEMP_ARCH_DIR}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST
mkdir /var/repository/repos/macos/$package/$type/$version-$number
cd /var/repository/repos/macos/$package/$type/$version-$number
put -r ${package}_"${version}".zip
ls -l /var/repository/repos/macos/$package/$type/$version-$number
EOF
