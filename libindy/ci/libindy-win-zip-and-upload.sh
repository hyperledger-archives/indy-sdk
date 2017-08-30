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

mkdir libindy-zip
mkdir libindy-zip/lib
cp -r ./include ./libindy-zip
cp ./target/release/*.dll ./libindy-zip/lib/
powershell.exe -nologo -noprofile -command "& { Add-Type -A 'System.IO.Compression.FileSystem'; [IO.Compression.ZipFile]::CreateFromDirectory('libindy-zip', 'libindy_$version.zip'); }"
rm -rf ./libindy-zip

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
mkdir /var/repository/repos/libindy/windows/$type/$version-$number
cd /var/repository/repos/libindy/windows/$type/$version-$number
put -r libindy_"$version".zip
ls -l /var/repository/repos/libindy/windows/$type/$version-$number
EOF
