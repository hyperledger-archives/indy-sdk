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

TEMP_ARCH_DIR=./indy-cli-zip
mkdir ${TEMP_ARCH_DIR}
cp ./target/release/*.dll ${TEMP_ARCH_DIR}/
cp ./target/release/indy-cli.exe ${TEMP_ARCH_DIR}/
powershell.exe -nologo -noprofile -command "& { Add-Type -A 'System.IO.Compression.FileSystem'; [IO.Compression.ZipFile]::CreateFromDirectory('${TEMP_ARCH_DIR}', 'indy-cli_$version.zip'); }"
rm -rf ${TEMP_ARCH_DIR}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115
mkdir /var/repository/repos/windows/indy-cli
mkdir /var/repository/repos/windows/indy-cli/master
mkdir /var/repository/repos/windows/indy-cli/rc
mkdir /var/repository/repos/windows/indy-cli/stable
mkdir /var/repository/repos/windows/indy-cli/$type/$version-$number
cd /var/repository/repos/windows/indy-cli/$type/$version-$number
put -r indy-cli_"$version".zip
ls -l /var/repository/repos/windows/indy-cli/$type/$version-$number
EOF
