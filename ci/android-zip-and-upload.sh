#!/bin/bash

set -e
set -x

if [ "$1" = "--help" ] ; then
  echo "Usage: <architecture> <version> <key> <branchName> <number> <artifact_name>"
  return
fi

arch="$1"
version="$2"
key="$3"
branchName="$4"
buildNumber="$5"
artifact="$6"

[ -z $arch ] && exit 1
[ -z $version ] && exit 2
[ -z $key ] && exit 3
[ -z $branchName ] && exit 4
[ -z $buildNumber ] && exit 5
[ -z $artifact ] && exit 6

ssh -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST mkdir -p /var/repository/repos/android/${artifact}/${branchName}/${version}-${buildNumber}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST
cd /var/repository/repos/android/${artifact}/${branchName}/$version-$buildNumber
put -r ${artifact}_android_${arch}_${version}.zip
ls -l /var/repository/repos/android/${artifact}/${branchName}/$version-$buildNumber
EOF
