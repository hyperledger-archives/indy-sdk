# Setup of IOS build environment.

1. Install [rustup](https://www.rustup.rs) toolchain manager.

2. Install toolchains using command 
   > rustup update aarch64-apple-ios
   > rustup update armv7-apple-ios
   > rustup update armv7s-apple-ios
   > rustup update i386-apple-ios
   > rustup update x86_64-apple-darwin
   > rustup update x86_64-apple-ios

3. Add line in Cargo.toml below libsovrin specification
   > crate-type = ["staticlib", "rlib"]

4. Set environment variables:
  > export PKG_CONFIG_ALLOW_CROSS=1
  > export OPENSSL_DIR=path_to_your_openssl
  
  for example
 
   > OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2k

5. Go to the folder with Cargo.toml file. Type:
  > cargo lipo  
