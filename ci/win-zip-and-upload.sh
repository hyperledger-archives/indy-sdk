#!/bin/bash

set -e
set -x

if [ "$1" = "--help" ] ; then
  echo "Usage: <folder> <package> <package_type> <version> <key> <type> <number>"
  return
fi

folder="$1"
package="$2"
package_type="$3"
version="$4"
key="$5"
type="$6"
number="$7"

[ -z $folder ] && exit 1
[ -z $package ] && exit 2
[ -z $package_type ] && exit 3
[ -z $version ] && exit 4
[ -z $key ] && exit 5
[ -z $type ] && exit 6
[ -z $number ] && exit 7

TEMP_ARCH_DIR=./${package}-zip
mkdir ${TEMP_ARCH_DIR}

if [ ${package_type} = "lib" ] ; then
  mkdir ${TEMP_ARCH_DIR}/lib
  cp -r ${folder}/include ${TEMP_ARCH_DIR}
  cp ./target/release/*.dll ${TEMP_ARCH_DIR}/lib/
  cp ./target/release/*.dll.lib ${TEMP_ARCH_DIR}/lib/
elif [ ${package_type} = "executable" ] ; then
  cp ./target/release/*.dll ${TEMP_ARCH_DIR}/
  cp ./target/release/${package}.exe ${TEMP_ARCH_DIR}/
else
  exit 2
fi

cd ${TEMP_ARCH_DIR} && zip -r ${package}_${version}.zip ./* && mv ${package}_${version}.zip .. && cd ..

rm -rf ${TEMP_ARCH_DIR}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST
mkdir /var/repository/repos/windows/$package/$type/$version-$number
cd /var/repository/repos/windows/$package/$type/$version-$number
put -r ${package}_"${version}".zip
ls -l /var/repository/repos/windows/$package/$type/$version-$number
EOF
