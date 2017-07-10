#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
ls -l /var/repository/repos
rm /var/repository/repos/rpm/*
cd /var/repository/repos/rpm/
put -r /usr/src/rpm/RPMS/
put -r /usr/src/rpm/SRPMS/
ls -l /var/repository/repos/rpm/
EOF