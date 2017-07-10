#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

echo "Uploading...."

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
ls -l /var/repository/repos
EOF