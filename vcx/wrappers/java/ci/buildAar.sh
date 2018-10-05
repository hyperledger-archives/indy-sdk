#!/usr/bin/env bash

SCRIPT_PATH=${BASH_SOURCE[0]}      # this script's name
SCRIPT_NAME=${SCRIPT_PATH##*/}       # basename of script (strip path)
SCRIPT_DIR="$(cd "$(dirname "${SCRIPT_PATH:-$PWD}")" 2>/dev/null 1>&2 && pwd)"

pushd ${SCRIPT_DIR} # we will work on relative paths from the script directory
    pushd ..
    ./gradlew --no-daemon clean build --project-dir=android -x test #skipping tests because the already run in jenkins CI
    mkdir -p artifacts/aar
    pushd android/build/outputs/aar
        cp $(ls -t1 |  head -n 1) ${SCRIPT_DIR}/../artifacts/aar
    popd

popd