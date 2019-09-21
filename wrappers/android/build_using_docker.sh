#!/usr/bin/env bash

pushd ../../
    pwd
    cp libindy/ci/ubuntu.dockerfile .
    cp wrappers/android/Dockerfile .
    echo "Building android wrapper for indy-sdk"
    # docker build -t indy-sdk-base -f ubuntu.dockerfile .
    # docker build -t indy-sdk-android .
    # docker rm indy-sdk-android
    docker run -it indy-sdk-android /bin/sh -c "cd wrappers/android;chmod +x gradlew;chmod +x build.sh; ./build.sh"

    echo "Copying file from container to indy-sdk/wrappers/android directory"
    docker cp indy-sdk-android:wrappers/android/build/outputs/aar/libindy-debug.aar .

    rm ubuntu.dockerfile
    rm Dockerfile
popd