#!/bin/sh

#1) Install Rust and rustup (https://www.rust-lang.org/install.html).
RUSTC_VERSION=`rustc --version`
if [ "$?" != "0" ]; then
    if [ -f $HOME/.cargo/bin/rustc ]; then
        echo "You need to add $HOME/.cargo/bin to your PATH environment variable or simply restart your terminal"
        exit 1
    else
        curl https://sh.rustup.rs -sSf | sh
        source $HOME/.cargo/env
        rustup component add rust-src
        rustup component add rust-docs
        rustup update
        RUSTC_VERSION=`rustc --version`
    fi
fi
# Steps to uninstall rustup to test that the step 1) works again
# rustup self uninstall

if [[ $RUSTC_VERSION =~ ^'rustc ' ]]; then
    rustup component add rls-preview rust-analysis rust-src
    rustup target add aarch64-apple-ios x86_64-apple-ios
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
fi