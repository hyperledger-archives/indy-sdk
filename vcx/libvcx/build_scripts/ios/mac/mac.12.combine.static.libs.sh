#!/bin/sh

# Combined all static libaries in the current directory into a single static library
# It is hardcoded to use the i386, armv7, and armv7s architectures; this can easily be changed via the 'archs' variable at the top
# The script takes a single argument, which is the name of the final, combined library to be created.
# If libvcxpartial is passed in as the parameter, only armv7 and arm64 are packaged 
#
#   For example:
#  =>    combine_static_libraries.sh combined-library
#
# Script by Evan Schoenberg, Regular Rate and Rhythm Software
# Thanks to Claudiu Ursache for his blog post at http://www.cvursache.com/2013/10/06/Combining-Multi-Arch-Binaries/ which detailed the technique automated by this script
#####
# $1 = Name of output archive
#####

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

INDY_SDK=$WORK_DIR/vcx-indy-sdk
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

cd $VCX_SDK/vcx/wrappers/ios/vcx/lib

if [ "$1" = "" ] || [ "$1" = "libvcx" ]; then
    echo "You must provide a name for the resultant library, the name libvcx is ALREADY used!"
    exit 1
fi

if [ "$2" = "delete" ]; then
    rm $1.a
fi

if [ -f $1.a ]; then
    echo "The library $1.a already exists!!!"
    exit 1
fi

DEBUG_SYMBOLS="debuginfo"
if [ ! -z "$3" ]; then
    DEBUG_SYMBOLS=$3
fi

IOS_ARCHS="armv7,armv7s,arm64,i386,x86_64"
if [ ! -z "$4" ]; then
    IOS_ARCHS=$4
fi
bkpIFS="$IFS"
IFS=',()][' read -r -a archs <<<"${IOS_ARCHS}"
echo "Combining architectures: ${archs[@]}"    ##Or printf "%s\n" ${array[@]}
IFS="$bkpIFS"

if [ "$1" = "libvcxpartial" ]; then
    archs=(armv7 arm64)
#else
#    archs=(armv7 armv7s arm64 i386 x86_64)
fi

libraries=(*.a)
libtool="/usr/bin/libtool"

echo "Combining ${libraries[*]}..."

for library in ${libraries[*]}
do
    lipo -info $library
    
    # Extract individual architectures for this library
    for arch in ${archs[*]}
    do
        lipo -extract $arch $library -o ${library}_${arch}.a
    done
done

# Combine results of the same architecture into a library for that architecture
source_combined=""
for arch in ${archs[*]}
do
    source_libraries=""
    
    for library in ${libraries[*]}
    do
        if [ "$DEBUG_SYMBOLS" = "nodebug" ]; then
            lipo ${library}_${arch}.a -thin $arch -output ${library}-$arch-unstripped.a
            strip -S -x -o ${library}-$arch-stripped.a -r ${library}-$arch-unstripped.a
            mv ${library}-$arch-stripped.a ${library}_${arch}.a
            rm ${library}-$arch-unstripped.a
        fi
        source_libraries="${source_libraries} ${library}_${arch}.a"
    done
    
    $libtool -static ${source_libraries} -o "${1}_${arch}.a"
    source_combined="${source_combined} ${1}_${arch}.a"
    
    # Delete intermediate files
    rm ${source_libraries}
done

# Merge the combined library for each architecture into a single fat binary
lipo -create $source_combined -o $1.a

# Delete intermediate files
rm ${source_combined}

# Show info on the output library as confirmation
echo "Combination complete."
lipo -info $1.a
