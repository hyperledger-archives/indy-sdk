#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

echo "Uploading...."

cat <<EOF | sftp -i $key repo@54.187.56.182
ls -l /var/repository/repos/rpm
rm /var/repository/repos/rpm/*
rmdir /var/repository/repos/rpm
mkdir /var/repository/repos/rpm/
cd /var/repository/repos/rpm/
put $WORK_DIR/$POD_FILE_NAME
put -r /usr/src/rpm/
ls -l /var/repository/repos/rpm/
EOF