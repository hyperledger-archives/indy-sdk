#!/bin/bash

onred='\033[41m'
ongreen='\033[42m'
onyellow='\033[43m'
endcolor="\033[0m"

# Handle errors
set -e
error_report() {
    echo -e "${onred}Error: failed on line $1.$endcolor"
}
trap 'error_report $LINENO' ERR

echo -e "${onyellow}Installing libindy...$endcolor"

if [[ "$OSTYPE" == "darwin"* ]]; then
    xcode-select --version || xcode-select --install
    brew --version || yes | /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
    cmake --version || brew install cmake # brew install cmake throws error, not warning if already installed
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH" # so can use cargo without relog
    brew install pkg-config \
                 https://raw.githubusercontent.com/Homebrew/homebrew-core/65effd2b617bade68a8a2c5b39e1c3089cc0e945/Formula/libsodium.rb \
                 automake \
                 autoconf \
                 openssl \
                 zeromq \
                 zmq
    export PKG_CONFIG_ALLOW_CROSS=1
    export CARGO_INCREMENTAL=1
    export RUST_LOG=indy=trace
    export RUST_TEST_THREADS=1
    for version in `ls -t /usr/local/Cellar/openssl/`; do
        export OPENSSL_DIR=/usr/local/Cellar/openssl/$version
        break
    done
    cargo build
    export LIBRARY_PATH=$(pwd)/target/debug
    cd ../cli
    cargo build
    echo 'export DYLD_LIBRARY_PATH='$LIBRARY_PATH'
export LD_LIBRARY_PATH='$LIBRARY_PATH >> ~/.bash_profile
    echo -e "${ongreen}Libindy installed.$endcolor"
else
    echo -e "${onred}You are not running MacOS. This is a MacOS installer.$endcolor"
fi
