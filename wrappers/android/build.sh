#!/usr/bin/env bash

mkdir -p src/main/jniLibs/armeabi-v7a
mkdir -p src/main/jniLibs/arm64
mkdir -p src/main/jniLibs/x86
copy_native_binaries(){
    cp libindy_arm/lib/libindy.so ../wrappers/android/src/main/jniLibs/armeabi-v7a/
    cp libindy_arm64/lib/libindy.so ../wrappers/android/src/main/jniLibs/arm64/
    cp libindy_x86/lib/libindy.so ../wrappers/android/src/main/jniLibs/x86/
}

pushd ../../libindy
    # sh build-libindy-android.sh
    copy_native_binaries
popd





cp -r ../java/src/main/java src/main/
cp -r ../java/src/main/resources src/main/


javaDir=src/main/java
javaFiles=`find $javaDir -type f -name "*.java"`

for f in $javaFiles; do sed -i 's/java.util.concurrent.CompletableFuture/java9.util.concurrent.CompletableFuture/g' "$f"; done

./gradlew clean assemble

cp build/outputs/aar/*-debug.aar ../../samples/android/WrapperTest/libindy-debug/
rm ../../samples/android/WrapperTest/libindy-debug/libindy-debug.aar
mv ../../samples/android/WrapperTest/libindy-debug/*-debug.aar ../../samples/android/WrapperTest/libindy-debug/libindy-debug.aar
