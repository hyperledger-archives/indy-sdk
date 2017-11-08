#!/bin/bash -xe

if [ "$1" = "--help" ] ; then
  echo "Usage: <version> <key> <type> <number>"
  return
fi

version="$1"
key="$2"
type="$3"
number="$4"

[ -z $version ] && exit 1
[ -z $key ] && exit 2
[ -z $type ] && exit 3
[ -z $number ] && exit 4

mkdir -p /usr/src/rpm/SOURCES/

sed \
	-e "s|@commit@|$commit|g" \
	-e "s|@version@|$version.$commit|g" \
	ci/indy-sdk.spec.in >indy-sdk.spec

chown root.root indy-sdk.spec

spectool -g -R indy-sdk.spec || exit 4
rpmbuild -ba indy-sdk.spec || exit 5

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
mkdir /var/repository/repos/libindy/rhel/$type/$version-$number
cd /var/repository/repos/libindy/rhel/$type/$version-$number
put -r /usr/src/rpm/RPMS/
put -r /usr/src/rpm/SRPMS/
ls -l /var/repository/repos/libindy/rhel/$type/$version-$number
EOF
