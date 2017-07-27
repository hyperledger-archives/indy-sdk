#!/usr/bin/env bash

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <key>"
fi

key="$1"

cd /wrappers/python
mkdir debs

version=$(grep -Po "(?<=version=')([0-9]|\.)*" setup.py)

[ -z $key ] && exit 1
[ -z $version ] && exit 2

fpm --input-type "python" \
    --output-type "deb" \
    --verbose \
    --architecture "amd64" \
    --name "python-indy-sdk" \
    --license "Apache License 2.0" \
    --python-package-name-prefix "python3" \
    --directories "This is the official SDK for Hyperledger Indy, which provides a distributed-ledger-based foundation for self-sovereign identity. The major artifact of the SDK is a c-callable library; there are
also convenience wrappers for various programming languages. All bugs, stories, and backlog for this project are managed through Hyperledger's Jira in project IS (note that regular Indy tickets are
in the INDY project instead...). Also, join us on Jira's Rocket.Chat at #indy-sdk to discuss." \
    --python-bin "/usr/bin/python3.6" \
    --exclude "*.pyc" \
    --exclude "*.pyo" \
    --maintainer "Hyperledger <hyperledger-indy@lists.hyperledger.org>" \
    --package "/home/indy-sdk/wrappers/python/debs" \
    "/home/indy-sdk/wrappers/python"

cat <<EOF | sftp -v -oStrictHostKeyChecking=no -i $key repo@192.168.11.111
mkdir /var/repository/repos/deb/python-indy-sdk
mkdir /var/repository/repos/deb/python-indy-sdk/$version
cd /var/repository/repos/deb/python-indy-sdk/$version
put -r /var/lib/jenkins/workspace/wrappers/python/debs/python-indy-sdk_"$version"_amd64.deb
ls -l /var/repository/repos/deb/python-indy-sdk/$version
EOF