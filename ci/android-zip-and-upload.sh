#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: <architecture> <version> <key> <branchName> <number>"
  return
fi
export ANDROID_BUILD_FOLDER="/tmp/android_build"

arch="$1"
version="$2"
key="$3"
branchName="$4"
buildNumber="$5"

[ -z $arch ] && exit 1
[ -z $version ] && exit 2
[ -z $key ] && exit 3
[ -z $branchName ] && exit 4
[ -z $buildNumber ] && exit 5


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



#Packaging and uploading Libindy

cd ${ANDROID_BUILD_FOLDER}/libindy_${arch} && zip -r libindy_android_${arch}_${version}.zip ./* && mv libindy_android_${arch}_${version}.zip .. && cd ..


cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115
cd /var/repository/repos/android/libindy/$branchName/$version-$buildNumber
put -r libindy_android_${arch}_"${version}".zip
ls -l /var/repository/repos/android/libindy/$branchName/$version-$buildNumber
EOF
