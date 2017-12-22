# Setup Indy SDK build environment for MacOS

1. Install Rust and rustup (https://www.rust-lang.org/install.html).
2. Install required native libraries and utilities (libsodium is added with URL to homebrew since version<1.0.15 is required)

   ```
   brew install pkg-config
   brew install https://raw.githubusercontent.com/Homebrew/homebrew-core/65effd2b617bade68a8a2c5b39e1c3089cc0e945/Formula/libsodium.rb   
   brew install automake 
   brew install autoconf
   brew install cmake
   brew install openssl
   ```
   

3. Setup environment variables:
   ```
   export PKG_CONFIG_ALLOW_CROSS=1
   export CARGO_INCREMENTAL=1
   export RUST_LOG=indy=trace
   export RUST_TEST_THREADS=1
   ```
4. Setup OPENSSL_DIR variable: path to installed openssl library
   ```
   export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2n   # path changes with version number
   ```
5. Checkout and build the library:
   ```
   git clone https://github.com/hyperledger/indy-sdk.git
   cd ./indy-sdk/libindy
   cargo build
   ```
