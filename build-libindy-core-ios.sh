#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

[ -z $key ] && exit 1

pwd

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
rm /var/repository/repos/deb/pods-ios/libindy-core/0.1.1/libindy-core-ios.tar.gz
cd /var/repository/repos/deb/pods-ios/libindy-core/0.1.1
put /home/indy/libindy-core-ios.tar.gz
EOF