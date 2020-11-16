# Building binaries of LibIndy for Android

**Not ready for production use! Not fully tested.**

Supported architectures are arm, armv7, arm64, x86 and x86_64

## Prerequisites

- Docker

## Dependencies
- The build scripts downloads the prebuilt dependencies while building. The prebuilt dependencies are available [here](https://github.com/evernym/indy-android-dependencies/tree/master/prebuilt)
- If you want build the dependencies by yourself the instructions for that can be found [here](https://github.com/evernym/indy-android-dependencies) 

## How to build.
- If on Ubuntu make sure you have these packages installed
    ```
    apt-get install -y \
             pkg-config \
             libssl-dev \
             libgmp3-dev \
             curl \
             build-essential \
             libsqlite3-dev \
             cmake \
             apt-transport-https \
             ca-certificates \
             wget \
             devscripts \
             libncursesw5-dev \
             libzmq3-dev \
             zip \
             unzip \
             jq
- Run `indy-sdk/libindy/build-libindy-android.sh` to build libindy for arm, arm64 and x86
    - This generates the libindy zip file with each architecture in the indy-sdk/libindy
    - You can also set the `LIBINDY_VERSION` environment variable to append version number to generated zip file.
- To generate the build for a single architecture run `android.build.sh`
    - e.g  `android.build.sh -d arm` . The flag `-d` will download the dependencies automatically
    - e.g  `android.build.sh arm <PATH_TO_OPENSSL> <PATH_TO_SODIUM> <PATH_TO_ZMQ>`. If `-d` flag is not passed you have to give paths to dependencies


## Usage 
- Unzip the generated library.
- Copy generated `indy-sdk/libindy/build_scripts/android/libindy_arm/libindy.so`, `indy-sdk/libindy/build_scripts/android/indy-android-dependencies/prebuild/sodium/libsodium_arm/lib/libsodium.so`, and `indy-sdk/libindy/build_scripts/android/indy-android-dependencies/prebuild/zmq/libzmq_arm/lib/libzmq.so` to the jniLibs/armeabi-v7a folder of your android project
- Copy the corresponding files for jniLibs/arm64-v8a and jniLibs/x86 (similar to step above)
    - `libindy.so` file is the dynamic library which is statically linked to its dependencies. This library can be loaded into apk without having dependencies along with it.
    - `libindy_shared.so` file is the dynamic library which is dynamically linked to its dependencies. you need to pass the dependencies into apk.
- In order to use the library in Android, you need to set the EXTERNAL_STORAGE environment variable and load the library using JNA
    
`Os.setenv("EXTERNAL_STORAGE", getExternalFilesDir(null).getAbsolutePath(), true);`

`System.loadLibrary("indy");`


## Notes:
The shared binary (libindy.so) of only **x86_64** architecture is **not** statically linked with its dependencies.

Make sure the Android app which is going to use libindy has permissions to write to external storage. 

Add following line to AndroidManifest.xml

`<uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE"/>`

Android emulator generally use x86 images

If you receive a JNA error, you may need to add additonal files into your jniLibs folder. 
- Add the correct version of libjnidispatch.so to the corresponding subfolder in jniLibs -> https://github.com/java-native-access/jna/tree/master/lib/native
- For example, download android-aarch64.jar and extract `libjnidispatch.so` file from it. Copy `libjnidispatch.so` file to the jniLibs/arm64-v8a subfolder
- NOTE: You need to download the correct version of libjnidispatch.so (tag 4.5.1 in the jna repo is the version accepted by Indy SDK v1.14.2)
- Add the dependency `implementation 'net.java.dev.jna:jna:4.5.1@aar` in build.gradle 

Make sure you have `libc++_shared.so` file in the corresponsind subfolder in jniLibs

##Known Issues

- The Android build does not successfully compile on OSX
    - It fails on the libzmq linking


# Building binaries of Libnullpay for Android

**Not ready for production use! Not fully tested.**

## Prerequisites

- Docker

## Dependencies
- Libindy for Android


## How to build.
- Unzip `libindy_android_<ARCH>_<VERSION>`
- Copy the extracted folder to `indy-sdk/libnullpay/`
- Run `indy-sdk/libnullpay/build-libnullpay-android.sh` to build libnullpay for arm, arm64 and x86
- To build for individual architecture, run `indy-sdk/libnullpay/android.build.sh -d arm <PATH_TO_LIBINDY>` to build libnullpay for arm
    - Or set env variable `INDY_DIR=<PATH_TO_LIBINDY>` and run `android.build.sh -d arm` to generate for arm
    - Set env variable `INDY_DIR=<PATH_TO_LIBINDY>` and run `android.build.sh -d arm64` to generate for arm64
    - Set env variable `INDY_DIR=<PATH_TO_LIBINDY>` and run `android.build.sh -d x86` to generate for x86
