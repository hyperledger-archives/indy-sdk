# Vcx SDK for Java and Android

##JAR

run `./gradlew clean build`

The jar will be present in `sdk/vcx/wrappers/java/vcx/build/libs`

## AAR

### Generate the AAR

 - Copy the binaries i.e `libvcx.so` to folder `sdk/vcx/wrappers/java/vcx/android/src/main/jniLibs/<ARCH>`.
    - Make sure the binaries are in correct architecture folders.
 - run `./gradlew clean build --project-dir=android` in folder `sdk/vcx/wrappers/java/vcx`

###Publishing the AAR
- run `./gradlew clean assemble --project-dir=android` in folder `sdk/vcx/wrappers/java/vcx`

Aar will be present in `sdk/vcx/wrappers/java/vcx/android/build/outputs/aar`
