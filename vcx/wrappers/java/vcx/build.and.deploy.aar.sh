#!/bin/bash

./gradlew publish

#This copies the built aar file to the sample project wrappertest present in the repo
#cp -v build/outputs/aar/vcx-debug.aar ../android/sample_app/wrappertest/vcx-debug