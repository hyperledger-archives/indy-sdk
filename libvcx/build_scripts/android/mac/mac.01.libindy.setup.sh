#!/bin/sh

DOCKER_CMD=`which docker`
if [ "$DOCKER_CMD" = "" ]; then
    echo "The android build depends on docker being installed, please install docker!!!"
    exit 1
fi

#1) Install Rust and rustup (https://www.rust-lang.org/install.html).
RUSTUP_VERSION=`rustup --version`
if [ "$?" != "0" ]; then
    if [ -f $HOME/.cargo/bin/rustup ]; then
        echo "You need to add $HOME/.cargo/bin to your PATH environment variable or simply restart your terminal"
        exit 1
    else
        if [ -f /usr/local/bin/rustc ]; then
            sudo mv /usr/local/bin/rustc /usr/local/bin/rustc.bak
            sudo mv /usr/local/bin/rustdoc /usr/local/bin/rustdoc.bak
            sudo mv /usr/local/bin/rust-lldb /usr/local/bin/rust-lldb.bak
            sudo mv /usr/local/bin/rust-gdb /usr/local/bin/rust-gdb.bak
        fi
        if [ -d /usr/local/lib/rustlib ]; then
            sudo mv /usr/local/lib/rustlib /usr/local/lib/rustlib.bak
            sudo mkdir /usr/local/lib/rustlib.bak/libs
            sudo mv /usr/local/lib/librustc* /usr/local/lib/rustlib.bak/libs
        fi    
        curl https://sh.rustup.rs -sSf | sh
        source $HOME/.cargo/env
        rustup component add rust-src
        rustup component add rust-docs
        rustup update
        RUSTUP_VERSION=`rustup --version`
        if [ -f /usr/local/bin/rustc.bak ]; then
            sudo mv /usr/local/bin/rustc.bak /usr/local/bin/rustc
            sudo mv /usr/local/bin/rustdoc.bak /usr/local/bin/rustdoc
            sudo mv /usr/local/bin/rust-lldb.bak /usr/local/bin/rust-lldb
            sudo mv /usr/local/bin/rust-gdb.bak /usr/local/bin/rust-gdb
        fi
        if [ -d /usr/local/lib/rustlib.bak ]; then
            sudo mv /usr/local/lib/rustlib.bak /usr/local/lib/rustlib
            sudo mv /usr/local/lib/rustlib/libs/* /usr/local/lib
            sudo rm -rf /usr/local/lib/rustlib/libs
        fi        
    fi
fi
# Steps to uninstall rustup to test that the step 1) works again
# rustup self uninstall

ANDROID_SDK_MANAGER=`which sdkmanager`
if [ "$ANDROID_SDK_MANAGER" != "" ]; then
        #/Users/norm/Library/Android/sdk/tools/bin/sdkmanager
        # This assumes that the android sdk is already installed, easiest way is via Android Studio
        NDKBUNDLE_DIR=`dirname $ANDROID_SDK_MANAGER`/../../ndk-bundle
        if [ ! -d $NDKBUNDLE_DIR ]; then
            sdkmanager --verbose ndk-bundle
            ./mac.build.ndk.standalone.toolchain.sh
            ./mac.libssl.libcrypto.build.sh
            ./mac.libzmq.libsodium.build.sh
            ./mac.build.libz.sh
        fi
else
    echo "ERROR: You must first install the android sdkmanager and set the environment variable ANDROID_HOME, the easiest way is via Android Studio!!"
fi

if [[ $RUSTUP_VERSION =~ ^'rustup ' ]]; then
    rustup component add rls-preview rust-analysis rust-src

    rustup target remove aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
    rustup target add aarch64-linux-android armv7-linux-androideabi arm-linux-androideabi i686-linux-android x86_64-linux-android
    
    cargo install cargo-lipo
    cargo install cargo-xcode
    
    BREW_VERSION=`brew --version`
    if ! [[ $BREW_VERSION =~ ^'Homebrew ' ]]; then
        /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
        brew doctor
        brew update
    fi
    
    #2) Install required native libraries and utilities (libsodium is added with URL to homebrew since version<1.0.15 is required)
    brew install pkg-config
    brew install https://raw.githubusercontent.com/Homebrew/homebrew-core/65effd2b617bade68a8a2c5b39e1c3089cc0e945/Formula/libsodium.rb   
    brew install automake 
    brew install autoconf
    brew install cmake
    brew install openssl
    brew install zmq
    brew install wget
    brew install truncate
    brew install libzip
fi