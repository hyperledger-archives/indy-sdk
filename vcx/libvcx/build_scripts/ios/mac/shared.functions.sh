#!/bin/sh

indyBranch=${INDY_BRANCH:?"Indy branch needs to be set i.e (stbale or master)"}
indyVersion=${INDY_VERSION:?"Indy version needs to be set"}
sovZip=${SOVTOKEN_ZIP:?"Sovtoken zip file needs to be set"}

export LIBSOVTOKEN_IOS_BUILD_URL="https://repo.sovrin.org/ios/libsovtoken/stable/${sovZip}"
export LIBINDY_IOS_BUILD_URL="https://repo.sovrin.org/ios/libindy/${indyBranch}/libindy-core/${indyVersion}/libindy.tar.gz"
export LIBNULLPAY_IOS_BUILD_URL="https://repo.sovrin.org/ios/libnullpay/${indyBranch}/libnullpay-core/${indyVersion}/libnullpay.tar.gz"

export LIBSOVTOKEN_FILE=$(basename ${LIBSOVTOKEN_IOS_BUILD_URL})
export LIBSOVTOKEN_VERSION=$(echo ${LIBSOVTOKEN_FILE} | cut -d'_' -f 2)
export LIBINDY_FILE=$(basename ${LIBINDY_IOS_BUILD_URL})
export LIBINDY_VERSION=$(basename $(dirname ${LIBINDY_IOS_BUILD_URL}))
export LIBNULLPAY_FILE=$(basename ${LIBNULLPAY_IOS_BUILD_URL})
export LIBNULLPAY_VERSION=$(basename $(dirname ${LIBNULLPAY_IOS_BUILD_URL}))

export BUILD_CACHE=~/.build_libvxc/ioscache
mkdir -p ${BUILD_CACHE}

function abspath() {
    # generate absolute path from relative path
    # $1     : relative filename
    # return : absolute path
    if [ -d "$1" ]; then
        # dir
        (cd "$1"; pwd)
    elif [ -f "$1" ]; then
        # file
        if [[ $1 = /* ]]; then
            echo "$1"
        elif [[ $1 == */* ]]; then
            echo "$(cd "${1%/*}"; pwd)/${1##*/}"
        else
            echo "$(pwd)/$1"
        fi
    fi
}
