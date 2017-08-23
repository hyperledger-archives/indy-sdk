#!/bin/bash -x

if [ "$1" = "--help" ] ; then
  echo "Usage: $0 <patch-version>"
fi

suffix="$1"

sed -i -E "s/version='([0-9,.]+).*/version='\1-$suffix',/" setup.py