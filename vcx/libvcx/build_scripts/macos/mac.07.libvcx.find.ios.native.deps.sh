#!/bin/sh

# for lib in `grep -B 3 "Finished release \[optimized\] target(s) in" ./mac.03.libindy.build.sh.out| \
# grep "Running"|awk 'BEGIN { FS="="; } {for(i = 1; i <= NF; i++) { if($i ~ /extern/) { print $(i+1); } }}'| \
# awk '{ if($1 ~ /\.rlib/) { print $1; } }'`
# do
#     LIBDIR=`dirname $lib`
#     LIBNAME=`basename $lib`
#     set -f
#     NAMEARRAY=(${LIBNAME//-/ })
#     echo "cp -n $LIBDIR/$LIBNAME $LIBDIR/${NAMEARRAY[0]}.a"
# done

for lib in `grep -B 3 "Finished release \[optimized\] target(s) in" ./mac.03.libindy.build.sh.out| \
grep "Running"|awk 'BEGIN { FS="="; } {for(i = 1; i <= NF; i++) { if($i ~ /native/) { print $(i+1); } }}'| \
awk '{ print $1; }'`
do
    #echo $lib
    LIBDIR=`dirname $lib`
    LIBNAME=`basename $lib`
    LIBNAME=${LIBNAME//\`/}
    #set -f
    #NAMEARRAY=(${LIBNAME//-/ })
    #echo "cp -n $LIBDIR/$LIBNAME $LIBDIR/${NAMEARRAY[0]}.a"
    echo "Build static .a library files: $LIBDIR/$LIBNAME"
    ls -al $LIBDIR/$LIBNAME
done


for lib in `grep -B 3 "Finished release \[optimized\] target(s) in" ./mac.06.libvcx.build.sh.out| \
grep "Running"|awk 'BEGIN { FS="="; } {for(i = 1; i <= NF; i++) { if($i ~ /native/) { print $(i+1); } }}'| \
awk '{ print $1; }'`
do
    #echo $lib
    LIBDIR=`dirname $lib`
    LIBNAME=`basename $lib`
    LIBNAME=${LIBNAME//\`/}
    #set -f
    #NAMEARRAY=(${LIBNAME//-/ })
    #echo "cp -n $LIBDIR/$LIBNAME $LIBDIR/${NAMEARRAY[0]}.a"
    echo "Build static .a library files: $LIBDIR/$LIBNAME"
    ls -al $LIBDIR/$LIBNAME
done


