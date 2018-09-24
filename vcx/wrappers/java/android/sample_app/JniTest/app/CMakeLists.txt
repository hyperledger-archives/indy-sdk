# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html

# Sets the minimum version of CMake required to build the native library.

cmake_minimum_required(VERSION 3.4.1)



add_library(lib_crypto STATIC IMPORTED)
set_target_properties(lib_crypto PROPERTIES IMPORTED_LOCATION
    prebuilt/static/libcrypto.a)

add_library(lib_ssl STATIC IMPORTED)
set_target_properties(lib_ssl PROPERTIES IMPORTED_LOCATION
    prebuilt/static/libssl.a)

add_library(lib_sodium SHARED IMPORTED)
set_target_properties(lib_sodium PROPERTIES IMPORTED_LOCATION
    prebuilt/shared/libsodium.so)

add_library(lib_zmq SHARED IMPORTED)
set_target_properties(lib_zmq PROPERTIES IMPORTED_LOCATION
    prebuilt/shared/libzmq.so)

add_library(lib_vcx SHARED IMPORTED)
set_target_properties(lib_vcx PROPERTIES IMPORTED_LOCATION
    prebuilt/shared/libvcx.so)


# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.

add_library( # Sets the name of the library.
             native-lib

             # Sets the library as a shared library.
             INTERFACE)

# Searches for a specified prebuilt library and stores the path as a
# variable. Because CMake includes system libraries in the search path by
# default, you only need to specify the name of the public NDK library
# you want to add. CMake verifies that the library exists before
# completing its build.


# Specifies libraries CMake should link to your target library. You
# can link multiple libraries, such as libraries you define in this
# build script, prebuilt third-party libraries, or system libraries.

target_link_libraries( # Specifies the target library.
                       #native-lib

                       # Links the target library to the log library
                       # included in the NDK.
                       lib-vcx
                       lib-zmq
                       lib-sodium
                       lib-ssl
                       lib-crypto
                       )