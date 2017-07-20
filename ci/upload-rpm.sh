#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
rm /var/repository/repos/rpm/RPMS/x86_64/*
rmdir /var/repository/repos/rpm/RPMS/x86_64
rmdir /var/repository/repos/rpm/RPMS
rm /var/repository/repos/rpm/SRPMS/*
rmdir /var/repository/repos/rpm/SRPMS
rm /var/repository/repos/rpm/rpm/BUILD/*
rmdir /var/repository/repos/rpm/rpm/BUILD
rmdir /var/repository/repos/rpm/rpm/BUILDROOT
rm /var/repository/repos/rpm/rpm/RPMS/x86_64/*
rmdir /var/repository/repos/rpm//rpm/RPMS/x86_64
rmdir /var/repository/repos/rpm//rpm/RPMS
rm /var/repository/repos/rpm/rpm/SOURCES/*
rmdir /var/repository/repos/rpm/rpm/SOURCES
rmdir /var/repository/repos/rpm/rpm/SPECS
rm /var/repository/repos/rpm/rpm/SRPMS/*
rmdir /var/repository/repos/rpm/rpm/SRPMS
EOF