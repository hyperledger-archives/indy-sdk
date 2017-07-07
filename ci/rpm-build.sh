#!/bin/bash

commit=$1

echo commit
echo 000
version=$(wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | tr -d '" ')
echo 111
[ -z $version ] && exit 1
[ -z $commit ] && exit 2
echo 222
sed \
	-e "s|@commit@|$commit|g" \
	-e "s|@version@|$version.$commit|g" \
	indy-sdk.spec.in >indy-sdk.spec
echo 333
spectool -g -R indy-sdk.spec || exit 3

echo 444
rpmbuild -ba indy-sdk.spec || exit 4

555
