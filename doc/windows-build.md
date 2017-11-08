# Setup Indy SDK build environment for Windows

## Get/build dependencies

All prebuilt can be downloaded from
https://repo.sovrin.org/windows/libindy/deps/

### Binary deps

- https://www.npcglib.org/~stathis/downloads/openssl-1.0.2k-vs2017.7z
- https://download.libsodium.org/libsodium/releases/libsodium-1.0.12-msvc.zip

### Source deps

- http://www.sqlite.org/2017/sqlite-amalgamation-3180000.zip
- https://github.com/miracl/milagro-crypto-c/
- https://github.com/evernym/libzmq-pw

### Build sqlite

Download http://www.sqlite.org/2017/sqlite-amalgamation-3180000.zip

Create empty static library project and add sqlite.c file and 2 headers from exctraced
archive. Then just build it.

### Build milagro-crypto-c

Checkout https://github.com/miracl/milagro-crypto-c/ repository.
- cmake -DBUILD_SHARED_LIBS=OFF -DCMAKE_BUILD_TYPE=Release -DCMAKE_POSITION_INDEPENDENT_CODE=ON -G "Visual Studio 15 2017 Win64" .
- open AMCL.sln
- disable custom build steps
- build it

### Build libzmq-pw

Checkout https://github.com/evernym/libzmq-pw repository.
- open builds/msvc/vs2017/libzmq.sln
- switch "draft API" and "libsodium" options on
- change "output file name" to $(TargetName)-pw
- build (it may print errors while
  building tests which can be ignored)

## Build

- Get binary dependencies (libamcl*, openssl, libsodium, libzmq, sqlite3).
- Put all *.{lib,dll} into one directory and headers into include/ subdirectory.
- Configure MSVS environment to privide 64-bit builds by execution of `vcvars64.bat`:
  
  ```
  "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\"vcvars64.bat
  ```
  
  Note that depending on the version of Visual Studio placement of vcvars64.bat can be different. For example, it can be
  `C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\amd64\vcvars64.bat`  
- execute "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvars64.bat"
- Point path to this directory using environment variables:
  - set INDY_PREBUILT_DEPS_DIR=C:\BIN\x64
  - set MILAGRO_DIR=C:\BIN\x64
  - set ZMQPW_DIR=C:\BIN\x64
  - set SODIUM_LIB_DIR=C:\BIN\x64
  - set OPENSSL_DIR=C:\BIN\x64
- set PATH to find .dlls:
  - set PATH=C:\BIN\x64\lib;%PATH%
- change dir to indy-client and run cargo (you may want to add --release --target x86_64-pc-windows-msvc keys to cargo)

## openssl-sys workaround

If your windows build fails complaining on gdi32.lib you should edit

```
  ~/.cargo/registry/src/github.com-*/openssl-sys-*/build.rs
```

and add

```
  println!("cargo:rustc-link-lib=dylib=gdi32");
```

to the end of main() function.

Then try to rebuild whole project.

## Run integration tests

* Start local nodes pool on `127.0.0.1:9701-9708` with Docker:
 
  ```     
  docker build -f ci/indy-pool.dockerfile -t indy_pool .
  docker run -itd -p 9701-9709:9701-9709 indy_pool
  ```          
 
  Please note that this port mapping between container and local host requires
  latest Docker for Windows (linux containers) and windows system with Hyper-V support.
  
  If you use some Docker distribution based on Virtual Box you can use Virtual Box's 
  port forwarding future to map 9701-9709 container ports to local 9701-9709 ports.
 
* Run tests
  
  ```
  RUST_TEST_THREADS=1 cargo test
  ```