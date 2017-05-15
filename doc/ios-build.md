# Setup of IOS build environment.

1. Install [rustup](https://www.rustup.rs) toolchain manager.

2. Install toolchains using command
   > rustup update aarch64-apple-ios
   > rustup update armv7-apple-ios
   > rustup update armv7s-apple-ios
   > rustup update i386-apple-ios
   > rustup update x86_64-apple-darwin
   > rustup update x86_64-apple-ios

3. Edit script build-libsovrin-core-ios.sh: set the following variables to fit your environment:
     > export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2k
     > export EVERNYM_REPO_KEY=~/Documents/EvernymRepo
     > export LIBSOVRIN_POD_VERSION=0.0.1
    
    OPENSSL_DIR - path to installed openssl library
    EVERNYM_REPO_KEY - path to file with private key to be authorized on deb server
    LIBSOVRIN_POD_VERSION - version of livsovrin-core pod to be built
    
4. Run the script. Validate the output that all goes well.

5. cd Podspec
6. Create directory with name defined in LIBSOVRIN_POD_VERSION
    mkdir LIBSOVRIN_POD_VERSION
7. copy libsovrin-core.podspec.json to that new directory from some previous version
8. edit this json -> change version field to LIBSOVRIN_POD_VERSION
9. add new directory and file inside to git repository.
10. commit to master branch
11. for all projects which using libsovrin-core do not forget to make
     > pod repo update
     > pod install


