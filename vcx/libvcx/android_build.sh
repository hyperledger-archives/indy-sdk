#!/usr/bin/env bash
if [ -d "${HOME}/.NDK_TOOLCHAINS" ]; then

    export NDK_TOOLCHAIN_DIR=${HOME}/.NDK_TOOLCHAINS
fi


if [[ -z "${NDK_TOOLCHAIN_DIR}"  ]]; then
    echo "NDK_TOOLCHAIN_DIR is not set. Exiting.... "
    echo "If you have not setup Toolchains then try running install_toolchains.sh."
    exit 1
fi

if [ "$1" == "aarm64" ]; then
    echo "Building for aarch64-linux-android"
    # Search for libindy in the preferred location.
    if [ -d "/usr/local/aarch64-linux-android/libindy" ]; then
        export LIBINDY_DIR=/usr/local/aarch64-linux-android/libindy
    fi
    if [[ -z "${LIBINDY_DIR}"  ]]; then
        echo "LIBINDY_DIR is not set. It is needed for building LibVCX "
        exit 1
    fi
    
    export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2l
    export AR=${NDK_TOOLCHAIN_DIR}/arm64/bin/aarch64-linux-android-ar
    export CC=${NDK_TOOLCHAIN_DIR}/arm64/bin/aarch64-linux-android-clang

    # build commands
    cargo clean --target aarch64-linux-android
    cargo build --target aarch64-linux-android --release --verbose
fi