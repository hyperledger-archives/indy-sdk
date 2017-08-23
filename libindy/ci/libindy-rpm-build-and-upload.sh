#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <commit> $1 <key> $2 <number>"
fi

commit="$1"
key="$2"
number="$3"

mkdir -p /usr/src/rpm/SOURCES/

version="0.1.1"

[ -z $version ] && exit 1
[ -z $commit ] && exit 2
[ -z $key ] && exit 3

sed \
	-e "s|@commit@|$commit|g" \
	-e "s|@version@|$version.$commit|g" \
	ci/indy-sdk.spec.in >indy-sdk.spec

chown root.root indy-sdk.spec

spectool -g -R indy-sdk.spec || exit 4
rpmbuild -ba indy-sdk.spec || exit 5
