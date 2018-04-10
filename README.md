# VCX
## Linux
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Install libindy (https://repo.evernym.com/libindy/).
3) Clone this repo to your local machine.
4) From the local repository run the following commands to verify everything works:
    ```
    $ cargo build
    $ cargo test
    ```
5) Currently developers are using intellij for IDE development (https://www.jetbrains.com/idea/download/) with the rust plugin (https://plugins.jetbrains.com/plugin/8182-rust).

"Everything is awesome when you're part of a team!" #TeamOneDirection

# Debians and Artifacts

**`libvcx_<ver>_amd.deb`**
- a debian that will install the .so library into /usr/lib, update `ldconfig`, and install provision script to `/usr/share/libvcx/`.
- Published to https://repo.corp.evernym.com/deb/pool/main/libc/libvcx/

**`vcx_<ver>.deb`**
- an unintelligent debian package that puts the nodejs package contents into a global node_modules location.

**`vcx<ver>.tgz`**
- target for the `$npm install vcx<ver>.tgz`

**`libvcx.tar.gz`**
- simple archive of libvcx.so and provision python script.

## OSX
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Install libindy (https://repo.evernym.com/libindy/).
    - As of now there is no distribution channel for OSX for LibIndy. [You have to build it manually.](https://github.com/hyperledger/indy-sdk/blob/master/doc/mac-build.md) 
    - Copy generated `libindy.dylib` file to `/usr/local/lib`
3) Clone this repo to your local machine.
4) Run `export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2o_1` in terminal. Make sure OPENSSL_DIR points to directory containing openssl binaries.
5) From the local repository run the following commands to verify everything works:
    ```
    $ cargo build
    ```
5) Tests are not working on OSX as of now.

## Android
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Clone this repo to your local machine.
3) Install libindy (https://repo.evernym.com/libindy/).
    - As of now there is no distribution channel for Android for LibIndy. You have to build it manually.
    - Copy generated `libindy.a` file to whatever location you want
    - in android_build.sh update LD_LIBRARY_PATH with the directory of libindy.a
4) Run `install_toolchains.s`. You need to run this once to setup toolchains for android
5) Run `android_build.sh aarm64` to build libvcx for aarm64 architecture.(Other architerctures will follow soon)
5) Tests are not working on OSX as of now.



