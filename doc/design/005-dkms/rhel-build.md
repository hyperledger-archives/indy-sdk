# Setup Indy SDK build environment for RHEL-based distributions
These instructions have been tested on:
- Amazon Linux 2017.03
- Fedora 27

Please follow the instructions appropriate for your distribution.

## Building `libindy`
### 1. Install Rust
Installation via `rustup` is recommended. Follow
[these instructions](https://www.rust-lang.org/install.html).

### 2. Install dependencies available in system repositories

For Amazon Linux 2017.03/CentOS/RHEL:
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

For Fedora 26/27/28:
```
dnf clean all
dnf upgrade -y
dnf groupinstall -y "Development Tools"
dnf install -y \
   wget \
   cmake \
   pkgconfig \
   openssl-devel \
   sqlite-devel
```

### 3. Build and install a modern version of `libsodium` from source
For Amazon Linux 2017.03 or other distributions without `libsodium` available in system repositories:
```
cd /tmp
curl https://download.libsodium.org/libsodium/releases/libsodium-1.0.14.tar.gz | tar -xz
cd /tmp/libsodium-1.0.14
./configure
make
make install
rm -rf /tmp/libsodium-1.0.14

export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib
```

For Fedora 26/27/28, `libsodium-1.0.14` is already available as a system package:
```
dnf install libsodium libsodium-devel
```

Versions of `libsodium` from `1.0.15` onwards miss the required `crypto_stream_aes128ctr_*` functions. Because of this:
- for distributions without `libsodium` in the system repositories: be advised that in the future the archive might be moved to `https://download.libsodium.org/libsodium/releases/old/libsodium-1.0.14.tar.gz`
- for Fedora and other distribution where `libsodium` is available: make sure that the available version is at most `1.0.14`; if it is more recent, switch to building it from source as explained above

### 4. Additional dependencies
For Fedora 26/27/28, you may also need to install `zeromq` (`libzmq`) before being able to successfully
build `libindy`:
```
dnf install zeromq zeromq-devel
```
If you discover that there are other dependencies not mentioned here, please open an issue.

### 5. Checkout and build the library
```
git checkout https://github.com/hyperledger/indy-sdk.git
cd ./indy-sdk/libindy
cargo build
```

## Building `indy-cli`
`indy-cli` is dependent on `libindy` and must be built before `indy-cli`.

After building `libindy`, run the following commands from the `indy-sdk` directory:
```
cd indy-cli
RUSTFLAGS="-L ../libindy/target/{BUILD_TYPE}" cargo build
```
In the above command, substitute `{BUILD_TYPE}` with `release` or `debug` as appropriate.

If you have installed `libindy.so` to a system-wide location and subsequently run `ldconfig`, you do not need
to specify the `RUSTFLAGS` environment variable as `rustc` should be able to find `libindy.so` without additional
help.

If not, however, `indy-cli` needs help to be able to find `libindy.so` while being built. Setting `LD_LIBRARY_PATH`
is only referenced at runtime and not at build time and is not helpful in this case. Specifying `RUSTFLAGS` in the
command above will tell `rustc` to also check `../libindy/target/{BUILD_TYPE}` for libraries.

## Running integration tests
### Starting up
Start local nodes pool on `127.0.0.1:9701-9708` with Docker:

```
docker build -f ci/indy-pool.dockerfile -t indy_pool .
docker run -itd -p 9701-9709:9701-9709 indy_pool
```

In some environments, this approach with mapping of local ports to container ports
can't be applied. Dockerfile `ci/indy-pool.dockerfile` supports optional `pool_ip` param
that allows changing ip of pool nodes in generated pool configuration. The following
commands allow to start local nodes pool in custom docker network and access this pool by
custom ip in docker network:

```
docker network create --subnet 10.0.0.0/8 indy_pool_network
docker build --build-arg pool_ip=10.0.0.2 -f ci/indy-pool.dockerfile -t indy_pool .
docker run -d --ip="10.0.0.2" --net=indy_pool_network indy_pool
```

This may be useful if you want to launch integration tests inside another container attached to
the same docker network.

### Run tests

```
RUST_TEST_THREADS=1 cargo test
```

It is possible to change ip of test pool by providing of TEST_POOL_IP environment variable:

```
RUST_TEST_THREADS=1 TEST_POOL_IP=10.0.0.2 cargo test
```

See [ci/amazon.dockerfile](https://github.com/hyperledger/indy-sdk/blob/master/libindy/ci/amazon.dockerfile) for example of Amazon Linux based environment creation in Docker.
