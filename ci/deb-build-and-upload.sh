#!/bin/bash

set -e
set -x

if [ "$1" = "--help" ] ; then
  echo "Usage: <package> <version> <key> <type> <suffix> <repo> <host> <key>"
  return
fi

package="$1"
version="$2"
type="$3"
suffix="$4"
repo="$5"
host="$6"
key="$7"

[ -z $package ] && exit 1
[ -z $version ] && exit 2
[ -z $type ] && exit 3
[ -z $suffix ] && exit 4
[ -z $repo ] && exit 5
[ -z $host ] && exit 6
[ -z $key ] && exit 7

sed -i -E -e 'H;1h;$!d;x' -e "s/$package ([(,),0-9,.]+)/$package ($version$suffix)/" debian/changelog

dpkg-buildpackage -tc

mkdir debs &&  mv ../*.deb ./debs/

./sovrin-packaging/upload_debs.py ./debs $repo $type --host $host --ssh-key $key
