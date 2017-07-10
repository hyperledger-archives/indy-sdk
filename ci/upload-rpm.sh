#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

echo "Uploading...."

cat <<EOF | sftp $key repo@35.166.202.228
ls -l /var/repository/repos
EOF