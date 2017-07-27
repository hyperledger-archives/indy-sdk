# Setup Indy SDK build environment for RHEL based distro (Amazon Linux 2017.03)

1. Install Rust and rustup (https://www.rust-lang.org/install.html).
1. Install required native libraries and utilities available in repos:
   
   ```
   yum clean all
   yum upgrade -y
   yum groupinstall -y "Development Tools"
   yum install -y \
       wget \
       cmake \
       pkgconfig \
       openssl-devel \
       sqlite-devel
   ```
1. Build and install modern version of libsodium from sources:
   
   ```
   cd /tmp
   curl https://download.libsodium.org/libsodium/releases/libsodium-1.0.12.tar.gz | tar -xz
   cd /tmp/libsodium-1.0.12
   ./configure
   make
   make install
   rm -rf /tmp/libsodium-1.0.12

   export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig
   export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib
   ```

1. Checkout and build the library:

   ```
   git checkout https://github.com/hyperledger/indy-sdk.git
   cd ./indy-sdk
   cargo build
   ```
1. Run integration tests:
    * Start local nodes pool on `10.0.0.2:9701-9708` with Docker:
      
      ```
      docker network create --subnet 10.0.0.0/8 pool_network
      docker build -f ci/indy-pool.dockerfile -t indy_pool .
      docker run -d --ip="10.0.0.2" --net=pool_network indy_pool
      ```
    * Run tests
      
      ```
      RUST_TEST_THREADS=1 cargo test
      ```

See [ci/amazon.dockerfile](https://github.com/hyperledger/indy-sdk/tree/master/ci/amazon.dockerfile) for example of Amazon Linux based environment creation in Docker.