#!/bin/sh

export LIBINDY_IOS_BUILD_URL="https://repo.sovrin.org/ios/libindy/stable/libindy-core/1.7.0/libindy.tar.gz"

export LIBINDY_FILE=$(basename ${LIBINDY_IOS_BUILD_URL})
export LIBINDY_VERSION=$(basename $(dirname ${LIBINDY_IOS_BUILD_URL}))

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
