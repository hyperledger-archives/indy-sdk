#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <commit> $1 <key> $2 <number>"
fi

commit="$1"
key="$2"
number="$3"

version=$(wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/libindy/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | tr -d '" ')

[ -z $version ] && exit 1
[ -z $commit ] && exit 2
[ -z $key ] && exit 3

dpkg-buildpackage -tc

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
mkdir /var/repository/repos/libindy/ubuntu/master/$version-$number
cd /var/repository/repos/libindy/ubuntu/master/$version-$number
put -r ../indy-sdk-dev_"$version"_amd64.deb
put -r ../indy-sdk_"$version"_amd64.deb
ls -l /var/repository/repos/libindy/ubuntu/master/$version-$number
EOF