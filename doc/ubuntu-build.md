# Setup Indy SDK build environment for Ubuntu based distro (Ubuntu 16.04).

1. Install Rust and rustup (https://www.rust-lang.org/install.html).
2. Install required native libraries and utilities:
   
   ```
   apt-get update && \
   apt-get install -y \
      build-essential \
      pkg-config \
      cmake \
      libzmq3-dev \
      libssl-dev \
      libsqlite3-dev \
      libsodium-dev
   ```
3. Checkout and build the library:
   
   ```
   git checkout https://github.com/hyperledger/indy-sdk.git
   cd ./indy-sdk
   cargo build
   ```
4. Run integration tests:
   * Start local nodes pool on `10.0.0.2:9701-9708` with Docker:
     
     ```
     docker network create --subnet 10.0.0.0/8 pool_network
     docker build -f ci/sovrin-pool.dockerfile -t sovrin_pool .
     docker run -d --ip="10.0.0.2" --net=pool_network sovrin_pool
     ```
   * Run tests
     
     ```
     RUST_TEST_THREADS=1 cargo test
     ```

See [ubuntu.dockerfile](https://github.com/hyperledger/indy-sdk/tree/master/ci/ubuntu.dockerfile) for example of Ubuntu based environment creation in Docker.