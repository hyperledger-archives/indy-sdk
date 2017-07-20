#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <commit> $1 <key>"
fi

commit="$1"
key="$2"

version=$(wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | tr -d '" ')

[ -z $version ] && exit 1
[ -z $commit ] && exit 2
[ -z $key ] && exit 3

dpkg-buildpackage

cd ci

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
mkdir /var/repository/repos/deb/indy-sdk
mkdir /var/repository/repos/deb/indy-sdk/$version
cd /var/repository/repos/deb/indy-sdk/$version
put -r /home/indy-sdk-dev_0.1.1_amd64.deb
put -r /home/indy-sdk_0.1.1_amd64.deb
ls -l /var/repository/repos/deb/indy-sdk/$version
EOF
