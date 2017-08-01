#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <commit> $1 <key> $2 <number>"
fi

commit="$1"
key="$2"
number="$3"

version=$(wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | tr -d '" ')

sed -i -E "s/version = \"([0-9,.]+).*\"/version = \"\1-$number\"/" Cargo.toml

[ -z $version ] && exit 1
[ -z $commit ] && exit 2
[ -z $key ] && exit 3

dpkg-buildpackage

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
mkdir /var/repository/repos/deb/indy-sdk
mkdir /var/repository/repos/deb/indy-sdk/$version-$number
cd /var/repository/repos/deb/indy-sdk/$version-$number
put -r /var/lib/jenkins/workspace/indy-sdk-dev_"$version"-"$number"_amd64.deb
put -r /var/lib/jenkins/workspace/indy-sdk_"$version"-"$number"_amd64.deb
ls -l /var/repository/repos/deb/indy-sdk/$version-$number
EOF
