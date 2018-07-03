#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: <folder> <package> <architecture> <version> <key> <branchName> <number>"
  return
fi

folder="$1"
package="$2"
arch="$3"
version="$4"
key="$5"
branchName="$6"
buildNumber="$7"

[ -z $folder ] && exit 1
[ -z $package ] && exit 2
[ -z $arch ] && exit 3
[ -z $version ] && exit 4
[ -z $key ] && exit 5
[ -z $branchName ] && exit 6
[ -z $buildNumber ] && exit 7


echo "get_triplet_from_arch called with args ${arch}"
if [ -z $arch ]; then
    echo "please provide the arch e.g arm, x86 or arm64"
    exit 1
fi
if [ $arch == "arm" ]; then
    export triplet="arm-linux-androideabi"
fi

if [ $arch == "x86" ]; then
    export triplet="i686-linux-android"
fi

if [ $arch == "arm64" ]; then
    export triplet="aarch64-linux-android"
fi


TEMP_ARCH_DIR=./${package}_android_${arch}_zip
mkdir ${TEMP_ARCH_DIR}

mkdir ${TEMP_ARCH_DIR}/lib
cp -r runtime_android_build/indy-sdk/${folder}/include ${TEMP_ARCH_DIR}
cp -v runtime_android_build/indy-sdk/${folder}/target/${triplet}/release/libindy.so ${TEMP_ARCH_DIR}/lib/
cp -v runtime_android_build/indy-sdk/${folder}/target/${triplet}/release/libindy.a ${TEMP_ARCH_DIR}/lib/

cd ${TEMP_ARCH_DIR} && zip -r ${package}_android_${arch}_${version}.zip ./* && mv ${package}_android_${arch}_${version}.zip .. && cd ..

rm -rf ${TEMP_ARCH_DIR}
#TODO: Move from test folder in final commit
cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115
cd /var/repository/repos/test/android/$package/$branchName/$version-$buildNumber-$arch
put -r ${package}_android_${arch}_"${version}".zip
ls -l /var/repository/repos/test/android/$package/$branchName/$version-$buildNumber-$arch
EOF