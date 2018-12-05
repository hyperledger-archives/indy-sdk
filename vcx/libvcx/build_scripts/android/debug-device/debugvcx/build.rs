use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("target={}", target);

    //println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/NDK/x86/sysroot/usr/lib");
    //println!("cargo:rustc-link-lib=static=stdc++");

    println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/NDK/x86/i686-linux-android/lib");
    println!("cargo:rustc-link-lib=static=stdc++");
    //println!("cargo:rustc-link-lib=dylib=c++_shared");
    // println!("cargo:rustc-link-lib=static=c++_static");
    // println!("cargo:rustc-link-lib=static=c++abi");
    // println!("cargo:rustc-link-lib=static=android_support");
    //println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/NDK/x86/i686-linux-android/lib");
    
    //println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/NDK/x86/lib/gcc/i686-linux-android/4.9.x");
    //println!("cargo:rustc-link-lib=static=gcc");


    // Including libraries as static .a files -- this does not work quiet yet
    println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/libvcx/target/{}/release", target);
    println!("cargo:rustc-link-lib=static=vcx");
    println!("cargo:rustc-link-search=native=/Users/norm/Library/Android/sdk/ndk-bundle/platforms/android-24/arch-x86/usr/lib");
    println!("cargo:rustc-link-lib=static=c");
    //println!("cargo:rustc-link-lib=static=m");
    //println!("cargo:rustc-link-lib=dylib=c++_shared");
    //println!("cargo:rustc-link-lib=static=stdc++");
    //println!("cargo:rustc-link-lib=static=ev");
    //println!("cargo:rustc-link-lib=static=z");
        
    println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/libz-android/zlib/lib/x86");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/libzmq-android/libsodium/libsodium_x86/lib");
    println!("cargo:rustc-link-lib=static=sodium");
    println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/libzmq-android/zmq/libzmq_x86/lib");
    println!("cargo:rustc-link-lib=static=zmq");
    println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/openssl_for_ios_and_android/output/android/openssl-x86/lib");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/openssl_for_ios_and_android/output/android/openssl-x86/lib");
    println!("cargo:rustc-link-lib=static=crypto");
    

    // Including libraries as dylib .so files -- this WORKS it produces a binary
    // println!("cargo:rustc-link-search=native=/Users/norm/Library/Android/sdk/ndk-bundle/platforms/android-23/arch-x86/usr/lib");
    // println!("cargo:rustc-link-lib=dylib=c");
    // println!("cargo:rustc-link-lib=dylib=m");
    // println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/vcx/libvcx/target/{}/release", target);
    // println!("cargo:rustc-link-lib=dylib=vcx");
    
    // println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/libz-android/zlib/lib/x86");
    // println!("cargo:rustc-link-lib=dylib=z");
    // println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/libzmq-android/libsodium/libsodium_x86/lib");
    // println!("cargo:rustc-link-lib=dylib=sodium");
    // println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/libzmq-android/zmq/libzmq_x86/lib");
    // println!("cargo:rustc-link-lib=dylib=zmq");
    // println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/openssl_for_ios_and_android/output/android/openssl-x86/lib");
    // println!("cargo:rustc-link-lib=dylib=ssl");
    // println!("cargo:rustc-link-search=native=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/openssl_for_ios_and_android/output/android/openssl-x86/lib");
    // println!("cargo:rustc-link-lib=dylib=crypto");
}


/*
Open the app/build.gradle file:

In the android/defaultConfig section, specify the usage of armeabi-v7a and the c++_shared standard library.

  ndk {
     abiFilters 'armeabi-v7a'
     stl 'c++_shared'
  }
In the android/defaultConfig/externalNativeBuild section add the -DANDROID_STL=c++shared argument to force CMake to use the right stl:

externalNativeBuild {
     cmake {
         cppFlags ""
         arguments "-DANDROID_STL=c++_shared"
     }
}
 */