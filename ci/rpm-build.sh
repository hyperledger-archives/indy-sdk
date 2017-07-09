#!/bin/bash

commit=$1

echo commit
version=$(wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | tr -d '" ')
echo 000
[ -z $version ] && exit 1
echo 111
[ -z $commit ] && exit 2
echo 222
mkdir -p /usr/src/rpm/SOURCES/
echo 333
cd ci

echo 444

sed \
	-e "s|@commit@|$commit|g" \
	-e "s|@version@|$version.$commit|g" \
	indy-sdk.spec.in >indy-sdk.spec
echo 555
spectool -g -R indy-sdk.spec || exit 3
echo 666
rpmbuild -ba indy-sdk.spec || exit 4
echo 777