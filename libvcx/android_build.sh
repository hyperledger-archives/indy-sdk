
if [ "$1" == "aarm64" ]; then
    echo "Building for aarch64-linux-android"
    # Link to static libindy library
    export LD_LIBRARY_PATH=/Users/abdussami/Work/binaries/libindy/aarm64/release
    
    export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2l
    export AR=${NDK_TOOLCHAIN_DIR}/arm64/bin/aarch64-linux-android-ar
    export CC=${NDK_TOOLCHAIN_DIR}/arm64/bin/aarch64-linux-android-clang

    # build commands
    # cargo clean --target aarch64-linux-android
    cargo build --target aarch64-linux-android --verbose
fi