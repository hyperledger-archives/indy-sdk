#!/bin/bash -xe

if [ "$1" = "--help" ] ; then
  echo "Usage: <version> <key> <type> <number> <package>"
  return
fi

version="$1"
key="$2"
type="$3"
number="$4"
package="$5"
dir=$(pwd)
result_dir=$(pwd)/rpms

[ -z $version ] && exit 1
[ -z $key ] && exit 2
[ -z $type ] && exit 3
[ -z $number ] && exit 4
[ -z $package ] && exit 5

sed \
	-e "s|@version@|$version|g" \
	-e "s|@dir@|$dir|g" \
	-e "s|@release@|$number|g" \
	-e "s|@result_dir@|$result_dir|g" \
    rpm/${package}.spec.in > ${package}.spec

mkdir ${result_dir}

fakeroot rpmbuild -ba ${package}.spec --nodeps || exit 7

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST
mkdir /var/repository/repos/rpm/$package/$type/$version-$number
cd /var/repository/repos/rpm/$package/$type/$version-$number
put -r ${result_dir}/*
ls -l /var/repository/repos/rpm/$package/$type/$version-$number
EOF