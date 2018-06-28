# Building binaries of LibIndy for Android

**Not ready for production use! Not fully tested.**

## Prerequisites

- Docker

## Dependencies
- The build scripts downloads the prebuilt dependencies while building. The prebuilt dependencies are available [here](https://github.com/evernym/indy-android-dependencies/tree/master/prebuilt)
- If you want build the dependencies by yourself the instructions for that can be found [here](https://github.com/evernym/indy-android-dependencies)

## How to build.
- Run `indy-sdk/libindy/build-libindy-android.sh` to build libindy for arm, arm64 and x86

## Usage 
- Copy generated `indy-sdk/libindy/build_scripts/android/libindy_<ARCHITECTURE>/libindy.so` to the jniLibs folder of your android project
- Load library using the JNA


## Notes:
Make sure the Android app which is going to use libindy has permissions to write to external storage. 

Add following line to AndroidManifest.xml

`<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE"/>`

Android emulator generally use x86 images

##Known Issues

- The Android build does not successfully compile on OSX
    - It fails on the libzmq linking

- If you are using Linux and want to build without docker, use the script`indy-sdk/libindy/build_scripts/android/build.withoutdocker.sh` .
 - usage e.g `./build.withoutdocker.sh -d x86 16 i686-linux-android` to download the prebuilt binaries and build for x86 using api level 16 with ABI i686-linux-android
 - e.g `./build.withoutdocker.sh x86 16 i686-linux-android openssl_x86 libsodium_x86 libzmq_x86` if you want to pass the dependencies to the script


# Building binaries of Libnullpay for Android

**Not ready for production use! Not fully tested.**

## Prerequisites

- Docker

## Dependencies
- Libindy for Android


## How to build.
- Copy the `indy-sdk/libindy/build_scripts/android/libindy_<ARCHITECTURE>` folders to `indy-sdk/libnullpay/build_scripts/android/`
- Run `indy-sdk/libnullpay/build-libnullpay-android.sh` to build libnullpay for arm, arm64 and x86



