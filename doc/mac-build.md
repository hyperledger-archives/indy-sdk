# Setup Indy SDK build environment for MacOS

## Build

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

## Run integration tests & sample code

Integration tests, platform specific unit tests for wrappers and [samples](../samples)
run out of the box in most Linux environments. OSX requires some extra configuration
to allow for differences in the way Docker works on Mac. Indy uses the node IP address
as a part of Node transaction in pool ledger and is present in Merkle tree hashes.

* Set up port mapping between container and local host:

  If you use some Docker distribution based on Virtual Box you can use Virtual Box's 
  port forwarding future to map 9701-9709 container ports to local 9701-9709 ports.
 
  If you use VMWare Fusion to run Docker locally, follow the instructions from
  https://medium.com/@tuweizhong/how-to-setup-port-forward-at-vmware-fusion-8-for-os-x-742ad6ca1344
  and add the following lines to _/Library/Preferences/VMware Fusion/vmnet8/nat.conf_:

  ```
  # Use these with care - anyone can enter into your VM through these...
  # The format and example are as follows:
  #<external port number> = <VM's IP address>:<VM's port number>
  #8080 = 172.16.3.128:80
  9709 = <your_docker_ip>:9709
  9707 = <your_docker_ip>:9707
  9702 = <your_docker_ip>:9702
  9701 = <your_docker_ip>:9701
  9708 = <your_docker_ip>:9708
  9703 = <your_docker_ip>:9703
  9704 = <your_docker_ip>:9704
  9706 = <your_docker_ip>:9706
  9705 = <your_docker_ip>:9705
  ```
  where <your_docker_ip> is your Docker host IP.

  Docker machine needs to be rebooted after these changes.

* Start local nodes pool on `127.0.0.1:9701-9708` with Docker:

  ```
  docker build -f ci/indy-pool.dockerfile -t indy_pool .
  docker run -itd -p 9701-9709:9701-9709 indy_pool
  ```

* Run tests

  ```
  RUST_TEST_THREADS=1 cargo test
  ```