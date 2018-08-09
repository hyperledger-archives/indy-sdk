#!/bin/sh

export PATH=/Users/androidbuild1/forge/work/code/evernym/sdk/.macosbuild/NDK/x86/bin:$PATH
cargo build --target i686-linux-android --release --verbose
#adb push ./target/i686-linux-android/release/debugvcx /sdcard/Download/
