#!/bin/bash

if [ "$1" = "--help" ] ; then
  echo "Usage: <architecture> <version> <key> <branchName> <number> <artifact_name> <is_stable>"
  return
fi

arch="$1"
version="$2"
key="$3"
branchName="$4"
buildNumber="$5"
artifact="$6"
is_stable="$7"

${arch:?'arch variable is not set'}
${version:?'version variable is not set'}
${key:?'key variable is not set'}
${branchName:?'branchName variable is not set'}
${buildNumber:?'buildNumber variable is not set'}
${artifact:?'artifact variable is not set'}
${is_stable:?'is_stable variable is not set'}

if [ "${is_stable}" == "1" ]; then
    ssh -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115 mkdir -p /var/repository/repos/android/${artifact}/stable/${version}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115
cd /var/repository/repos/android/${artifact}/stable/$version
cp -vr /var/repository/repos/android/${artifact}/rc/$version/${artifact}/${artifact}_android_${arch}_${version}.zip .
ls -l /var/repository/repos/android/${artifact}/stable/$version
EOF
    else
    ssh -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115 mkdir -p /var/repository/repos/android/${artifact}/${branchName}/${version}-${buildNumber}

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.115
cd /var/repository/repos/android/${artifact}/rc/$version-$buildNumber
put -r ${artifact}/${artifact}_android_${arch}_${version}.zip
ls -l /var/repository/repos/android/${artifact}/rc/$version-$buildNumber
EOF
fi
