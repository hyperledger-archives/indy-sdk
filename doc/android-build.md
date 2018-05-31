# Building binaries of LibIndy for Android

**Not ready for production use! Not fully tested.**

## Prerequisites

- Docker

## How to build.
- Goto `indy-sdk/libindy/build_scripts/android` folder
- Run `build.dependencies.locally.sh`
    - This will locally build the dependencies for libindy
- Run `build_arm.sh` to build for armv7 cpu architecture
- Run `build_arm64.sh` to build for arm64 cpu architecture
- Run `build_x86.sh` to build for x86 cpu architecture

## Usage 
- Copy generated `indy-sdk/libindy/build_scripts/android/libindy.so` to the jniLibs folder of you android project
- Load library using the JNA


## Notes:
Make sure the Android app which is going to use libindy has permissions to write to external storage. 

Add following line to AndroidManifest.xml

`<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE"/>`

Android emulator generally use x86 images

##Known Issues

- The android build does successfully compile on OSX
    - It fails on the libzmq linking
    - `libindy/build_scripts/android/build.nondocker.sh` can be used to make android builds without Docker


