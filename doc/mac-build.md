# Setup of MAC build environment.

1. Install [rustup](https://www.rustup.rs) toolchain manager.

2. Install required native libraries and utilities
   > brew install libsodium
   > brew install cmake
   > brew install automake 
   > brew install autoconf
   > brew install openssl

3. Setup environment variables:
  
   > export PKG_CONFIG_ALLOW_CROSS=1
   > export CARGO_INCREMENTAL=1
   > export RUST_LOG=sovrin=trace
   > export RUST_TEST_THREADS=1

4. Setup OPENSSL_DIR variable: path to installed openssl library
   
   > export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2k
   
5. Build the library:
   
   > cargo build


