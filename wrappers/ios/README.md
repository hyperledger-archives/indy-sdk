# How to install
A wrapper is a private pod, so private podspec must be set. Put at the top of the Podfile: 
    
    source 'https://github.com/hyperledger/indy-sdk.git'
    
Cocoapod will search for spec files in the root Specs folder.

Add pod to target:
    
    pod 'libindy-objc'                

# How to build

1. Install Rust and rustup (https://www.rust-lang.org/install.html).
1. Install toolchains using command:

   ```
   rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios i386-apple-ios x86_64-apple-ios
   ```
1. Install cargo-lipo:
   
   ```
   cargo install cargo-lipo
   ```
1. Install required native libraries and utilities:
   
   ```
   brew install libsodium
   brew install zeromq
   brew install cmake
   ```
1. Setup environment variables:
   
   ```
   export PKG_CONFIG_ALLOW_CROSS=1
   export CARGO_INCREMENTAL=1
   ```
1. Edit script build-libindy-core-ios.sh: set the following variables to fit your environment:
   
   ```
   export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2k
   export EVERNYM_REPO_KEY=~/Documents/EvernymRepo
   export LIBINDY_POD_VERSION=0.0.1
   ```
   OPENSSL_DIR - path to installed openssl library
   
   EVERNYM_REPO_KEY - path to file with private key to be authorized on deb server
   
   LIBINDY_POD_VERSION - version of libindy-core pod to be built
1. Run the script. Validate the output that all goes well.
1. Go to `Podspec` dir.
1. Create directory with name defined in LIBINDY_POD_VERSION:
   
   ```
   mkdir LIBINDY_POD_VERSION
   ```
1. Copy libindy-core.podspec.json to that new directory from some previous version.
1. Edit this json -> change version field to LIBINDY_POD_VERSION.
1. Add new directory and file inside to git repository.
1. Commit to master branch.
1. for all projects which using libindy-core do not forget to make:

   ```
   pod repo update
   pod install
   ```
   
   
## Wrapper Cocoapod

# Creation 
Run Archive process for `Indy` target. Custom post-action shell script `universal_framework.h` will be triggered and you get universal framework. Then put it to folder: `libindy-objc/Indy.framework` and upload to repo.

# Usage

Import header starting from 0.1.3:

```
#import <Indy/Indy.h> 
```
For 0.1.1 and 0.1.2 versions:

```
#import <libindy/libindy.h>
```

All wrapper types and classes have prefix `Indy`.

#### Troubleshooting
Use environment variable `RUST_LOG={info|debug|trace}` to output logs of Libindy.