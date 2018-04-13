# VCX
## Linux
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Install libindy (https://repo.sovrin.org/sdk/deb/pool/xenial/stable/libi/libindy/)
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
        - Or create a symlink in `/usr/local/lib` pointing to newly generated `libindy.dylib`, this will help in updating the libindy in future.
3) Clone this repo to your local machine.
4) Run `export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2o_1` in terminal.
    - The version of openssl in /usr/local/Cellar/openssl may change. Set OPENSSL_DIR to the version installed on your Mac. 
    For example, run export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2o_1 in terminal if version 1.0.2o_1 is installed.
5) From the local repository run the following commands to verify everything works:
    ```
    $ cargo build
    ```
6) Make sure all tests are passing
       ```
       $ cargo test
       ```
       If tests do not pass due to a "signal: 11, SIGSEGV: invalid memory reference" or if you have intermittent behavior (some tests pass on one try then fail on the next) , try replacing the `rustc` binary installed in step 1 in ${HOME}/.cargo/bin with a version built from the stable branch.
   
       ```
       git clone git@github.com:rust-lang/rust.git -b stable
       cd rust
       ./x.py build && sudo ./x.py install
       ```
   
      This will install rustc, rust-gdb, rust-lldb, and rustdoc executables in /usr/local/lib
   
       Compare the version of ${HOME}/.cargo/bin/rustc with the version in /usr/local/lib
       ```
        ${HOME}/.cargo/bin/rustc --version
        /usr/local/bin/rustc --version
       ```
       It is likely that the only difference you will see in the version number is a "-dev" appended to the version in /usr/local/bin.
   
       Both ${HOME}/.cargo/bin and /usr/local/lib are likely in your PATH. It appears that when ${HOME}/.cargo/bin/cargo is used, the ${HOME}/.cargo/bin/rustc executable is used instead of /usr/local/lib/rustc, even if /usr/local/lib is first in your PATH. To fix this problem, simply delete rustc from ${HOME}/.cargo/bin. Doing so will ensure the versions in /usr/local/bin will be invoked.
   
       ```
       rm -f ${HOME}/.cargo/bin/rustc
       rm -f ${HOME}/.cargo/bin/rust-gdb
       rm -f ${HOME}/.cargo/bin/rust-lldb
       rm -f ${HOME}/.cargo/bin/rustdoc
       ```
       
       If this seems too messy to you, it is recommended that ${HOME}/.cargo be removed entirely (as if you never followed install instructions found at https://www.rust-lang.org/install.html) and build/install rust and cargo from source. To build and install cargo from source, follow instructions found at: https://github.com/rust-lang/cargo

## Android
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Clone this repo to your local machine.
3) Install libindy (https://repo.evernym.com/libindy/).
    - As of now there is no distribution channel for Android for LibIndy. You have to build it manually.
    - Copy generated `libindy.a` file to whatever location you want
    - Set env variable `LIBINDY_DIR=<Directory_containing_libindy.a>`. e.g `export LIBINDY_DIR=/usr/local/aarch64-linux-android/libindy` libindy directory holds libindy.a
4) Run `install_toolchains.sh`. You need to run this once to setup toolchains for android
5) Run `android_build.sh aarm64` to build libvcx for aarm64 architecture.(Other architerctures will follow soon)
6) Tests are not working on Android as of now.



