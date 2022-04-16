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

function brew_install {
    if brew ls --versions $1 >/dev/null; then
        if [[ $(brew outdated $1) ]]; then
            HOMEBREW_NO_AUTO_UPDATE=1 brew upgrade $1
        fi
    else
        HOMEBREW_NO_AUTO_UPDATE=1 brew install $1
    fi
}

if [[ "$OSTYPE" == "darwin"* ]]; then
    xcode-select --version || xcode-select --install
    brew --version || yes | /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
    cmake --version || brew install cmake # brew install cmake throws error, not warning if already installed
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH" # so can use cargo without relog
    brew_install pkg-config
    brew_install libsodium
    brew_install automake
    brew_install autoconf
    brew_install openssl@1.1
    brew_install zeromq
    brew_install zmq
    export PKG_CONFIG_ALLOW_CROSS=1
    export CARGO_INCREMENTAL=1
    export RUST_LOG=indy=trace
    export RUST_TEST_THREADS=1
    export OPENSSL_DIR=/usr/local/opt/`ls /usr/local/opt/ | grep openssl | sort | tail -1`
    cargo build
    export LIBRARY_PATH=$(pwd)/target/debug
    cd ../cli
    cargo build
    echo -e "" >> ~/.bash_profile
    echo -e "# Hyperledger Indy" >> ~/.bash_profile
    echo -e "export DYLD_LIBRARY_PATH=$LIBRARY_PATH" >> ~/.bash_profile
    echo -e "export LD_LIBRARY_PATH=$LIBRARY_PATH" >> ~/.bash_profile
    echo -e "${ongreen}Libindy installed.$endcolor"
else
    echo -e "${onred}You are not running MacOS. This is a MacOS installer.$endcolor"
fi
