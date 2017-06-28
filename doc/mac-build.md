# Setup Indy SDK build environment for MacOS

1. Install Rust and rustup (https://www.rust-lang.org/install.html).
1. Install required native libraries and utilities

   ```
   brew install libsodium
   brew install zeromq
   brew install cmake
   brew install openssl
   ```
1. Setup environment variables:
   ```
   export PKG_CONFIG_ALLOW_CROSS=1
   export CARGO_INCREMENTAL=1
   export RUST_LOG=sovrin=trace
   export RUST_TEST_THREADS=1
   ```
1. Setup OPENSSL_DIR variable: path to installed openssl library
   ```
   export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2k
   ```
1. Checkout and build the library:
   ```
   git checkout https://github.com/hyperledger/indy-sdk.git
   cd ./indy-sdk
   cargo build
   ```