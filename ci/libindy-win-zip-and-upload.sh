#!/bin/bash -e

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <commit> $1 <key> $2 <number>"
fi

commit="$1"
key="$2"
number="$3"

version=$(wget -q https://raw.githubusercontent.com/hyperledger/indy-sdk/$commit/libindy/Cargo.toml -O - | grep -E '^version =' | head -n1 | cut -f2 -d= | tr -d '" ')

[ -z $version ] && exit 1
[ -z $commit ] && exit 2
[ -z $key ] && exit 3

mkdir indy-sdk-zip
cd indy-sdk-zip
cp -r ../libindy/include .
mkdir lib
cp ../libindy/target/release/*.dll lib/
cd ..
powershell.exe -nologo -noprofile -command "& { Add-Type -A 'System.IO.Compression.FileSystem'; [IO.Compression.ZipFile]::CreateFromDirectory('indy-sdk-zip', 'indy-sdk_${version}.zip'); }"
rm -rf ./indy-sdk-zip

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
-mkdir /var/repository/repos/libindy/windows-bins
-mkdir /var/repository/repos/libindy/windows-bins/indy-sdk
mkdir /var/repository/repos/libindy/windows-bins/indy-sdk/$version-$number
cd /var/repository/repos/libindy/windows-bins/indy-sdk/$version-$number
put -r indy-sdk_"$version".zip
ls -l /var/repository/repos/libindy/windows-bins/indy-sdk/$version-$number
EOF
