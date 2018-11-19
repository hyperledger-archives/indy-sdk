#!/bin/sh

#
## Strip debug symbol from universal static libraries
#

if [ -z "$1" ]
then
	echo "$0 library"
	exit 1
fi

file "$1"|grep 'universal binary' >/dev/null 2>&1
if [ $? -ne 0 ]
then
	echo "$1: not a static library"
	exit 1
fi

TMP=`mktemp -d /tmp/tmp.XXXXXX`

for arch in `file "$1"|grep 'architecture '|sed 's/.*(for architecture \(.*\)).*/\1/'`
do
    #if [ "$arch" = "armv7" ] || [ "$arch" = "arm64" ]; then
        lipo "$1" -thin $arch -output $TMP/libfoo-$arch-unstripped.a
        strip -S -x -o $TMP/libfoo-$arch.a -r $TMP/libfoo-$arch-unstripped.a
    #fi
done

rm -f $TMP/*-unstripped.a

lipo -create -output "$1-stripped" $TMP/*.a

rm -f $TMP/*.a
rmdir $TMP
