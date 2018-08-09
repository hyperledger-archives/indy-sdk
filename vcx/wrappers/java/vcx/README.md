## Vcx SDK for Java and Android

###Development:
With Docker

- run `build_scripts/android/vcx/build.sh`

Without Docker

- run `build_scripts/android/vcx/build.nondocker.sh`

### Generate the AAR

 - copy generated `libvcx.so` to `sdk/vcx/wrappers/java/vcx/src/main/jniLibs/<ARCH` folder
 - run `./gradlew clean assemble` in folder `sdk/vcx/wrappers/java/vcx`

###Publishing the AAR
- run `./gradlew clean assemble` in folder `sdk/vcx/wrappers/java/vcx`
