#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

if [ ! -d $WORK_DIR/rust-src ]; then
    git clone git@github.com:rust-lang/rust.git -b stable $WORK_DIR/rust-src
fi
cd $WORK_DIR/rust-src
#./x.py clean && ./x.py build && ./x.py install
#./x.py build && ./x.py install
./x.py build
#./x.py build && sudo ./x.py install

#echo "Version of ${HOME}/.cargo/bin/rustc"
#${HOME}/.cargo/bin/rustc --version
#echo "Version of /usr/local/bin/rustc"
#/usr/local/bin/rustc --version

#rustup toolchain list
#rustup toolchain link customrust $WORK_DIR/rust-src/build/x86_64-apple-darwin/stage0/
#rustup toolchain link customrust $WORK_DIR/rust-src/build/x86_64-apple-darwin/stage2/
#rustup toolchain link customrust /Users/norm/forge/work/code/evernym/sdk-evernym/.macosbuild/rust-src/build/x86_64-apple-darwin/stage0/
#rustup toolchain link customrust /Users/norm/forge/work/code/evernym/sdk-evernym/.macosbuild/rust-src/build/x86_64-apple-darwin/stage2/
#rustup default customrust
#rustup default stable
#rustup update
#rustup component add rls-preview rust-analysis rust-src
#rustup target add aarch64-apple-ios x86_64-apple-ios
#cargo install cargo-lipo

# DO NOT DO THESE STEPS - THESE STEPS SEEM TO CAUSE PROBLEMS
# mv -f ${HOME}/.cargo/bin/rustc ${HOME}/.cargo/bin/rustc.bak
# rm ${HOME}/.cargo/bin/rustc
# mv -f ${HOME}/.cargo/bin/rust-gdb ${HOME}/.cargo/bin/rust-gdb.bak
# rm ${HOME}/.cargo/bin/rust-gdb
# mv -f ${HOME}/.cargo/bin/rust-lldb ${HOME}/.cargo/bin/rust-lldb.bak
# rm ${HOME}/.cargo/bin/rust-lldb
# mv -f ${HOME}/.cargo/bin/rustdoc ${HOME}/.cargo/bin/rustdoc.bak
# rm ${HOME}/.cargo/bin/rustdoc

# /usr/local/lib/rustlib/uninstall.sh
# curl https://sh.rustup.rs -sSf | sh

# mv ${HOME}/.cargo/bin/rustc.bak ${HOME}/.cargo/bin/rustc
# mv ${HOME}/.cargo/bin/rust-gdb.bak ${HOME}/.cargo/bin/rust-gdb
# mv ${HOME}/.cargo/bin/rust-lldb.bak ${HOME}/.cargo/bin/rust-lldb
# mv ${HOME}/.cargo/bin/rustdoc.bak ${HOME}/.cargo/bin/rustdoc

# DO NOT DO THESE STEPS - THESE STEPS SEEM TO CAUSE PROBLEMS
# mv /usr/local/bin/rustc /usr/local/bin/rustc.bak
# mv /usr/local/bin/rust-gdb /usr/local/bin/rust-gdb.bak
# mv /usr/local/bin/rust-lldb /usr/local/bin/rust-lldb.bak
# mv /usr/local/bin/rustdoc /usr/local/bin/rustdoc.bak
