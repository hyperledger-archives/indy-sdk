#!/usr/bin/env bash



WORKDIR=${PWD}
LIBINDY_WORKDIR=${WORKDIR}
CI_DIR="${LIBINDY_WORKDIR}/ci"
export ANDROID_BUILD_FOLDER="/tmp/android_build"

TARGET_ARCH=$1

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi

set -e

source ${CI_DIR}/setup.android.env.sh
generate_arch_flags ${TARGET_ARCH}

echo ">> in runner script"
WORKDIR=${PWD}
declare -a EXE_ARRAY

build_test_artifacts(){
    pushd ${WORKDIR}

        set -e
        cp "${TOOLCHAIN_PREFIX}/android-ndk-r16b/platforms/android-21/arch-${TARGET_ARCH}/usr/lib/libc.so" "${TOOLCHAIN_DIR}/sysroot/usr/lib"
        cargo clean

        EXE_ARRAY=($( RUSTFLAGS="-L${TOOLCHAIN_DIR}/sysroot/usr/lib -lc -lz -L${TOOLCHAIN_DIR}/${TRIPLET}/lib -L${LIBZMQ_LIB_DIR} -L${SODIUM_LIB_DIR} -lsodium -lzmq -lgnustl_shared" \
                    cargo test --release --target=${TRIPLET} --no-run --message-format=json | jq -r "select(.profile.test == true) | .filenames[]"))
    popd
}

create_cargo_config(){
mkdir -p ${LIBINDY_WORKDIR}/.cargo
cat << EOF > ${LIBINDY_WORKDIR}/.cargo/config
[target.${TRIPLET}]
ar = "$(realpath ${AR})"
linker = "$(realpath ${CC})"
EOF
}

execute_on_device(){

    adb push \
    "${TOOLCHAIN_DIR}/${TRIPLET}/lib/libgnustl_shared.so" "/data/local/tmp/libgnustl_shared.so"

    adb push \
    "${SODIUM_LIB_DIR}/libsodium.so" "/data/local/tmp/libsodium.so"

    adb push \
    "${LIBZMQ_LIB_DIR}/libzmq.so" "/data/local/tmp/libzmq.so"

    for i in "${EXE_ARRAY[@]}"
    do
       :
        EXE="${i}"
        EXE_NAME=`basename ${EXE}`


        adb push "$EXE" "/data/local/tmp/$EXE_NAME"
        adb shell "chmod 755 /data/local/tmp/$EXE_NAME"
        OUT="$(mktemp)"
        MARK="ADB_SUCCESS!"
        adb shell "TEST_POOL_IP=10.0.0.2 LD_LIBRARY_PATH=/data/local/tmp RUST_TEST_THREADS=1 RUST_LOG=debug /data/local/tmp/$EXE_NAME && echo $MARK" 2>&1 | tee $OUT
        grep $MARK $OUT
    done

}



download_sdk
download_and_unzip_dependencies ${ABSOLUTE_ARCH}
download_and_setup_toolchain
set_env_vars
create_standalone_toolchain_and_rust_target
create_cargo_config
build_test_artifacts &&
check_if_emulator_is_running &&
execute_on_device
kill_avd
