#!/bin/bash
printenv
curl https://sh.rustup.rs -sSf > rustup-init.sh
chmod a+x rustup-init.sh
./rustup-init.sh -y
source .cargo/env
python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${TARGET_ARCH} --api ${TARGET_API} --install-dir ${TOOLCHAIN_DIR}
cat << EOF > .cargo/config
[target.${CROSS_COMPILE}]
ar = "${AR}"
linker = "${CC}"
EOF

export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_INCREMENTAL=1
export RUST_LOG=indy=trace
export RUST_TEST_THREADS=1
export RUST_BACKTRACE=1

rustup target add ${CROSS_COMPILE}

cd "${HOME}/indy-sdk/libnullpay"

cargo build --release --target=${CROSS_COMPILE}

cp "${HOME}/indy-sdk/libnullpay/target/${CROSS_COMPILE}/release/libnullpay.so" ${HOME}/
cp "${HOME}/indy-sdk/libnullpay/target/${CROSS_COMPILE}/release/libnullpay.a" ${HOME}/
