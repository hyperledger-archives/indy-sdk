#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <commit> $1 <file>"
fi

commit="$1"
file="$2"

mkdir -p /usr/src/rpm/SOURCES/

version=$(wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | tr -d '" ')

[ -z $version ] && exit 1
[ -z $commit ] && exit 2

cd ci

sed \
	-e "s|@commit@|$commit|g" \
	-e "s|@version@|$version.$commit|g" \
	indy-sdk.spec.in >indy-sdk.spec

chown root.root $file

spectool -g -R indy-sdk.spec || exit 3
rpmbuild -ba indy-sdk.spec || exit 4