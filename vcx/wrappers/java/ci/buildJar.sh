#!/usr/bin/env bash

set -e
version="$1"
SCRIPT_PATH=${BASH_SOURCE[0]}      # this script's name
SCRIPT_NAME=${SCRIPT_PATH##*/}       # basename of script (strip path)
SCRIPT_DIR="$(cd "$(dirname "${SCRIPT_PATH:-$PWD}")" 2>/dev/null 1>&2 && pwd)"

pushd ${SCRIPT_DIR} # we will work on relative paths from the script directory
    pushd ..
    PACKAGE_VERSION=$version ./gradlew --no-daemon clean build -x test #skipping tests because they already run in jenkins CI
    mkdir -p artifacts/jar
    pushd build/libs
        cp $(ls -t1 |  head -n 1) ${SCRIPT_DIR}/../artifacts/jar
    popd

popd