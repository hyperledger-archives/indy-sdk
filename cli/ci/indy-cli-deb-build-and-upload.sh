#!/bin/bash -xe

if [ "$1" = "--help" ] ; then
  echo "Usage: <version> <key> <type> <number>"
  return
fi

version="$1"
type="$2"
suffix="$3"
repo="$4"
host="$5"
key="$6"

[ -z $version ] && exit 1
[ -z $type ] && exit 2
[ -z $suffix ] && exit 3
[ -z $repo ] && exit 4
[ -z $host ] && exit 5
[ -z $key ] && exit 6

sed -i -E -e 'H;1h;$!d;x' -e "s/indy-cli ([(,),0-9,.]+)/indy-cli ($version$suffix)/" debian/changelog

dpkg-buildpackage -tc

mkdir debs &&  mv ../*.deb ./debs/

./sovrin-packaging/upload_debs.py ./debs $repo $type --host $host --ssh-key $key
