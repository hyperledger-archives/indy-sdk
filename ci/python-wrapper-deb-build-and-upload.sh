#!/usr/bin/env bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

mkdir /home/indy/debs

version=$(grep -Po "(?<=version=')([0-9]|\.|-)*" setup.py)
license=$(grep -Po "(?<=license=').[^\']*" setup.py)
description=$(grep -Po "(?<=description=').[^\']*" setup.py)

[ -z $key ] && exit 1
[ -z $version ] && exit 2

echo "Building...."

fpm --input-type "python" \
    --output-type "deb" \
    --verbose \
    --architecture "amd64" \
    --name "python-indy-sdk" \
    --license license \
    --python-package-name-prefix "python3" \
    --directories description \
    --python-bin "/usr/bin/python3.6" \
    --exclude "*.pyc" \
    --exclude "*.pyo" \
    --maintainer "Hyperledger <hyperledger-indy@lists.hyperledger.org>" \
    --package "/home/indy/debs" \
    .

echo "Uploading...."

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
mkdir /var/repository/repos/deb/python-indy-sdk
mkdir /var/repository/repos/deb/python-indy-sdk/$version
cd /var/repository/repos/deb/python-indy-sdk/$version
put -r /home/indy/debs/python-indy-sdk_"$version"_amd64.deb
ls -l /var/repository/repos/deb/python-indy-sdk/$version
EOF

