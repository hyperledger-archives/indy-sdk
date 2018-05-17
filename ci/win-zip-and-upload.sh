#!/bin/bash -xe

if [ "$1" = "--help" ] ; then
  echo "Usage: <package> <package_type> <version> <key> <type> <number>"
  return
fi

package="$1"
package_type="$2"
version="$3"
key="$4"
type="$5"
number="$6"

[ -z $package ] && exit 1
[ -z $package_type ] && exit 2
[ -z $version ] && exit 3
[ -z $key ] && exit 4
[ -z $type ] && exit 5
[ -z $number ] && exit 6

TEMP_ARCH_DIR=./${package}-zip
mkdir ${TEMP_ARCH_DIR}

if [ ${package_type} = "lib" ] ; then
  mkdir ${TEMP_ARCH_DIR}/lib
  cp -r ./include ${TEMP_ARCH_DIR}
  cp ./target/release/*.dll ${TEMP_ARCH_DIR}/lib/
elif [ ${package_type} = "executable" ] ; then
  cp ./target/release/*.dll ${TEMP_ARCH_DIR}/
  cp ./target/release/${package}.exe ${TEMP_ARCH_DIR}/
else
  exit 2
fi

powershell.exe -nologo -noprofile -command "& { Add-Type -A 'System.IO.Compression.FileSystem'; [IO.Compression.ZipFile]::CreateFromDirectory('${TEMP_ARCH_DIR}', '${package}_${version}.zip'); }"
rm -rf ${TEMP_ARCH_DIR}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115
mkdir /var/repository/repos/windows/$package/$type/$version-$number
cd /var/repository/repos/windows/$package/$type/$version-$number
put -r ${package}_"${version}".zip
ls -l /var/repository/repos/windows/$package/$type/$version-$number
EOF