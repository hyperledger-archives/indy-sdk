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

[ -z $version ] && exit 1
[ -z $key ] && exit 2
[ -z $type ] && exit 3
[ -z $number ] && exit 4
[ -z $package ] && exit 5

mkdir -p /usr/src/rpm/SOURCES/

sed \
	-e "s|@version@|$version|g" \
	-e "s|@dir@|$dir|g" \
    rpm/${package}.spec.in > ${package}.spec

mkdir rpms

spectool -R ${package}.spec || exit 6
rpmbuild -ba ${package}.spec || exit 7

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@$SOVRIN_REPO_HOST
mkdir /var/repository/repos/centos/$package/$type/$version-$number
cd /var/repository/repos/centos/$package/$type/$version-$number
put -r /rpms/x86_64/
ls -l /var/repository/repos/centos/$package/$type/$version-$number
EOF