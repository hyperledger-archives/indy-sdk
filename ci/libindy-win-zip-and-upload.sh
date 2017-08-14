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
cp -r libindy/include .
mkdir lib
cp libindy/target/release/*.dll lib/
cd ..
powershell.exe -nologo -noprofile -command "& { Add-Type -A 'System.IO.Compression.FileSystem'; [IO.Compression.ZipFile]::CreateFromDirectory('indy-sdk-zip', 'indy-sdk_${version}.zip'); }"
rm -rf ./indy-sdk-zip

echo "-mkdir /var/repository/repos/deb/windows-bins/" >> upload.sh
echo "-mkdir /var/repository/repos/deb/windows-bins/indy-sdk" >> upload.sh
echo "mkdir /var/repository/repos/deb/windows-bins/indy-sdk/$version-$number" >> upload.sh
echo "cd /var/repository/repos/deb/windows-bins/indy-sdk/$version-$number" >> upload.sh
echo "put -r indy-sdk_"$version".zip" >> upload.sh
echo "ls -l /var/repository/repos/deb/windows-bins/indy-sdk/$version-$number" >> upload.sh

sftp -v -oStrictHostKeyChecking=no -i $key -b upload.sh repo@192.168.11.111
