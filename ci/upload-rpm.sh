#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
ls -l /var/repository/repos
rm /var/repository/repos/rpm/rpm*
rmdir /var/repository/repos/rpm/rpm
EOF