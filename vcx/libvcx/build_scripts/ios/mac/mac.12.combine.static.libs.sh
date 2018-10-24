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

set -e
source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

INDY_SDK=$WORK_DIR/vcx-indy-sdk
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")

cd $VCX_SDK/vcx/wrappers/ios/vcx/lib

COMBINED_LIB=$1
if [ "${COMBINED_LIB}" = "" ] || [ "${COMBINED_LIB}" = "libvcx" ]; then
    echo "You must provide a name for the resultant library, the name libvcx is ALREADY used!"
    exit 1
fi

if [ "$2" = "delete" ]; then
    rm -rf ${COMBINED_LIB}.a
fi

if [ -f ${COMBINED_LIB}.a ]; then
    echo "The library ${COMBINED_LIB}.a already exists!!!"
    exit 1
fi

DEBUG_SYMBOLS="debuginfo"
if [ ! -z "$3" ]; then
    DEBUG_SYMBOLS=$3
fi

IOS_ARCHS="arm64,armv7,armv7s,i386,x86_64"
if [ ! -z "$4" ]; then
    IOS_ARCHS=$4
fi
bkpIFS="$IFS"
IFS=',()][' read -r -a archs <<<"${IOS_ARCHS}"
echo "Combining architectures: ${archs[@]}"    ##Or printf "%s\n" ${array[@]}
IFS="$bkpIFS"

# if [ "${COMBINED_LIB}" = "libvcxpartial" ]; then
#     archs=(armv7 arm64)
#else
#    archs=(armv7 armv7s arm64 i386 x86_64)
# fi

libraries=(*.a.tocombine)
libtool="/usr/bin/libtool"
mkdir -p ${BUILD_CACHE}/arch_libs

echo "Combining ${libraries[*]}..."

for library in ${libraries[*]}
do
    lipo -info $library

    # TEMPORARY HACK (build libvcx without duplicate .o object files):
    # There are duplicate .o object files inside the libvcx.a file and these
    # lines of logic remove those duplicate .o object files
    rm -rf ./tmpobjs
    mkdir ./tmpobjs
    pushd ./tmpobjs
        ar -x ../${library}
        ls > ../objfiles
        xargs ar cr ../${library}.new < ../objfiles
        if [ "$DEBUG_SYMBOLS" = "nodebug" ]; then
            ${STRIP} -S -x -o ../${library}.stripped -r ../${library}.new
            mv ../${library}.stripped ../${library}
        else
            mv ../${library}.new ../${library}
        fi
    popd

    # Extract individual architectures for this library
    for arch in ${archs[*]}
    do
        if [ "${library}" = "libvcx.a.tocombine" ]; then
            rm -rf ${BUILD_CACHE}/arch_libs/${library}_${arch}.a
            lipo -extract $arch $library -o ${BUILD_CACHE}/arch_libs/${library}_${arch}.a
        elif [ ! -f ${BUILD_CACHE}/arch_libs/${library}_${arch}.a ]; then
            lipo -extract $arch $library -o ${BUILD_CACHE}/arch_libs/${library}_${arch}.a
        fi
    done
done

# Combine results of the same architecture into a library for that architecture
source_combined=""
for arch in ${archs[*]}
do
    source_libraries=""

    for library in ${libraries[*]}
    do
        # if [ "$DEBUG_SYMBOLS" = "nodebug" ]; then
        #     if [ "${library}" = "libvcx.a.tocombine" ]; then
        #         rm -rf ${BUILD_CACHE}/arch_libs/${library}-$arch-stripped.a
        #         strip -S -x -o ${BUILD_CACHE}/arch_libs/${library}-$arch-stripped.a -r ${BUILD_CACHE}/arch_libs/${library}_${arch}.a
        #     elif [ ! -f ${BUILD_CACHE}/arch_libs/${library}-$arch-stripped.a ]; then
        #         strip -S -x -o ${BUILD_CACHE}/arch_libs/${library}-$arch-stripped.a -r ${BUILD_CACHE}/arch_libs/${library}_${arch}.a
        #     fi
        #     #mv ${library}-$arch-stripped.a ${library}_${arch}.a
        #     source_libraries="${source_libraries} ${BUILD_CACHE}/arch_libs/${library}-$arch-stripped.a"
        # else
            source_libraries="${source_libraries} ${BUILD_CACHE}/arch_libs/${library}_${arch}.a"
        # fi
    done

    echo "Using source_libraries: ${source_libraries} to create ${BUILD_CACHE}/arch_libs/${COMBINED_LIB}_${arch}.a"
    rm -rf "${BUILD_CACHE}/arch_libs/${COMBINED_LIB}_${arch}.a"
    $libtool -static ${source_libraries} -o "${BUILD_CACHE}/arch_libs/${COMBINED_LIB}_${arch}.a"
    source_combined="${source_combined} ${BUILD_CACHE}/arch_libs/${COMBINED_LIB}_${arch}.a"

    # Delete intermediate files
    #rm ${source_libraries}
done

echo "Using source_combined: ${source_combined} to create ${COMBINED_LIB}.a"
# Merge the combined library for each architecture into a single fat binary
lipo -create $source_combined -o ${COMBINED_LIB}.a

# Delete intermediate files
rm -rf ${source_combined}

# Show info on the output library as confirmation
echo "Combination complete."
lipo -info ${COMBINED_LIB}.a
