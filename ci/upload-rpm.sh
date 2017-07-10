#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

echo "Uploading...."

cat <<EOF | sftp $key repo.evernym.com
ls -l /var/repository/repos
EOF