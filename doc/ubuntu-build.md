# Setup Indy SDK build environment for Ubuntu based distro (Ubuntu 16.04)

1. Install Rust and rustup (https://www.rust-lang.org/install.html).
1. Install required native libraries and utilities:
   
   ```
   apt-get update && \
   apt-get install -y \
      build-essential \
      pkg-config \
      cmake \
      libssl-dev \
      libsqlite3-dev \
      libsodium-dev
   ```
1. Checkout and build the library:
   
   ```
   git clone https://github.com/hyperledger/indy-sdk.git
   cd ./indy-sdk/libindy
   cargo build
   cd ..
   ```
1. Run integration tests:
   * Start local nodes pool on `127.0.0.1:9701-9708` with Docker:
     
     ```     
     docker build -f ci/indy-pool.dockerfile -t indy_pool .
     docker run -itd -p 9701-9708:9701-9708 indy_pool
     ```     
     
     In some environments, this approach with mapping of local ports to container ports
     can't be applied. Dockerfile `ci/indy-pool.dockerfile` supports optional pool_ip param
     that allows changing ip of pool nodes in generated pool configuration. The following
     commands allow to start local nodes pool in custom docker network and access this pool by
     custom ip in docker network:
     
     ```
     docker network create --subnet 10.0.0.0/8 indy_pool_network
     docker build --build-arg pool_ip=10.0.0.2 -f ci/indy-pool.dockerfile -t indy_pool .
     docker run -d --ip="10.0.0.2" --net=indy_pool_network indy_pool
     ```
     
     It can be useful if we want to launch integration tests inside another container attached to
     the same docker network. 
     
   * Run tests
     
     ```
     cd libindy
     RUST_TEST_THREADS=1 cargo test
     ```
     
     It is possible to change ip of test pool by providing of TEST_POOL_IP environment variable:
     
     ```
     RUST_TEST_THREADS=1 TEST_POOL_IP=10.0.0.2 cargo test
     ```
     
See [ci/ubuntu.dockerfile](https://github.com/hyperledger/indy-sdk/tree/master/ci/ubuntu.dockerfile) for example of Ubuntu based environment creation in Docker.
