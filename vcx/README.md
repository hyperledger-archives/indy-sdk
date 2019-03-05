# VCX

Libvcx is a c-callable library built on top of Libindy that provides a high-level credential exchange protocol. 
It simplifies creation of agent applications and provides better agent-2-agent interoperability for [Hyperledger Indy](https://www.hyperledger.org/projects/hyperledger-indy)
infrastructure.

**Note** This library is currently in an **experimental** state.

## Installing the VCX
* VCX requires access to some Cloud Agent for full work. 
[Here](https://github.com/hyperledger/indy-sdk/tree/master/vcx/dummy-cloud-agent/README.md) is the simple Agent that can be used.
* VCX requires some payment plugin.
[Here](https://github.com/hyperledger/indy-sdk/tree/master/libnullpay/README.md) is the simple plugin that can be used.

### Ubuntu based distributions (Ubuntu 16.04)
It is recommended to install the VCX packages with APT:

    sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial {release channel}"
    sudo apt-get update
    sudo apt-get install -y libvcx

{release channel} must be replaced with master, rc or stable to define corresponded release channel.
Please See the section [Release channels](../README.md/#release-channels) for more details.

### Windows

1. Go to https://repo.sovrin.org/windows/libvcx/{release-channel}.
2. Download last version of libvcx.
3. Unzip archives to the directory where you want to save working library.
4. After unzip you will get next structure of files:

* `Your working directory`
    * `include`
        * `vcx.h`
    * `lib`
        * `vcx.dll`
        * `nullpay.dll`
        * `indy.dll`
        * `libeay32md.dll`
        * `libsodium.dll`
        * `libzmq.dll`
        * `ssleay32md.dll`

`include` contains c-header file which contains all necessary declarations
that may be need for your applications.

`lib` contains all necessary binaries which contains libvcx and all it's dependencies.
 `You must add to PATH environment variable path to lib`. It's necessary for dynamic linkage
 your application with libvcx.

{release channel} must be replaced with master, rc or stable to define corresponded release channel.

### OSX

To build libvcx for OSX and iOS using scripts do the following steps --
1) Add the following environment variables to your .bash_profile  
export PKG_CONFIG_ALLOW_CROSS=1  
export CARGO_INCREMENTAL=1  
export RUST_LOG=indy=trace  
export RUST_TEST_THREADS=1  
for i in `ls -t /usr/local/Cellar/openssl/`; do export OPENSSL_DIR=/usr/local/Cellar/openssl/$i; break; done  
export PYTHONPATH=/Users/[your_username]/[path_to_sdk]/vcx/libvcx/vcx-indy-sdk/wrappers/python:/Users/[your_username]/[path_to_sdk]/vcx/wrappers/python3:${PYTHONPATH}  
`# it is important that the $HOME/.cargo/bin comes first in the PATH`  
export PATH="$HOME/.cargo/bin:$PATH"  
export PKG_CONFIG_PATH=/usr/lib/pkgconfig:/usr/local/Cellar/zeromq/4.2.5/lib/pkgconfig:/usr/local/Cellar/libsodium/1.0.12/lib/pkgconfig  
2) git clone this repository
3) cd sdk/vcx/libvcx/build/macos
4) ./mac.01.libindy.setup.sh
5) source ./mac.02.libindy.env.sh
6) ./mac.03.libindy.build.sh > mac.03.libindy.build.sh 2<&1
7) ./mac.04.libvcx.setup.sh
8) source ./mac.05.libvcx.env.sh
9) ./mac.06.libvcx.build.sh > mac.06.libvcx.build.sh.out 2>&1
10) If the script ./mac.06.libvcx.build.sh terminates with the message
"signal: 11, SIGSEGV: invalid memory reference" OR "signal: 4, SIGILL: illegal instruction"
then that means the 'cargo test' command was unsuccessful OR if you have intermittent
behavior (some tests pass on one try then fail on the next) with the
'cargo test' command then execute the script ./mac.build.and.install.rust.tools.sh
After the mac.build.and.install.rust.tools.sh finishes (it will take a long long time)
then restart your terminal and then re-run all of the scripts starting at step 1)
above and they should all be successful. If they are not successful then run the
./mac.build.and.install.rust.tools.sh script one more time (it will be fast this time)
and DO NOT restart your terminal but run the ./mac.06.libvcx.build.sh script and
it should finish successfully.
11) ./mac.07.libvcx.prepare.ios.deps.sh


To build libvcx on your own you can follow these steps --
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Install or build libindy (https://repo.evernym.com/libindy/).
    - As of now there is no distribution channel for OSX for LibIndy. [You have to build it manually.](https://github.com/hyperledger/indy-sdk/blob/master/docs/source/build-guides/mac-build.md) 
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
       ./x.py build && ./x.py install
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

### Android
1) Install rust and rustup (https://www.rust-lang.org/install.html).
2) Clone this repo to your local machine.
3) Install libindy (https://repo.evernym.com/libindy/).
    - As of now there is no distribution channel for Android for LibIndy. You have to build it manually.
    - Copy generated `libindy.a` file to whatever location you want
    - Set env variable `LIBINDY_DIR=<Directory_containing_libindy.a>`. e.g `export LIBINDY_DIR=/usr/local/aarch64-linux-android/libindy` libindy directory holds libindy.a
4) Run `install_toolchains.sh`. You need to run this once to setup toolchains for android
5) Run `android_build.sh aarm64` to build libvcx for aarm64 architecture.(Other architerctures will follow soon)
6) Tests are not working on Android as of now.

## How to build VCX from source

## Linux 
1) Install rust and rustup (https://www.rust-lang.org/install.html). 
2) [Install Libindy](../README.md#installing-the-sdk) 
3) Optionally [install Libnullpay](../libnullpay/README.md) to include payment functionality.
3) Clone this repo to your local machine. 
4) From the indy-sdk/vcx/libvcx folder inside this local repository run the following commands to verify everything works: 
    ``` 
    $ cargo build 
    $ cargo test 
    ``` 
5) Currently developers are using intellij for IDE development (https://www.jetbrains.com/idea/download/) with the rust plugin (https://plugins.jetbrains.com/plugin/8182-rust). 
 
## Wrappers documentation

The following wrappers are tested and complete.

* [Java](wrappers/java/README.md)
* [Python](wrappers/python3/README.md)
* [iOS](wrappers/ios/README.md)
* [NodeJS](wrappers/node/README.md)

## Getting started guide
[The tutorial](docs/source/getting-started/getting-started.md) which introduces Libvcx and explains how the whole ecosystem works, and how the functions in the SDK can be used to construct rich clients.

### Example use
For the main workflow example check [demo](https://github.com/hyperledger/indy-sdk/tree/master/vcx/wrappers/python3/demo).
