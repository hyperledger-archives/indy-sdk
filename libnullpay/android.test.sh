#!/usr/bin/env bash



WORKDIR="$( cd "$(dirname "$0")" ; pwd -P )"
LIBINDY_WORKDIR="$(realpath ${WORKDIR}/../libindy)"
LIBNULLPAY_WORKDIR=${WORKDIR}
CI_DIR="${LIBINDY_WORKDIR}/ci"
export ANDROID_BUILD_FOLDER="/tmp/android_build"

TARGET_ARCH=$1

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi

if [ -z "${INDY_DIR}" ] ; then
        INDY_DIR="libindy_${TARGET_ARCH}"
        if [ -d "${INDY_DIR}" ] ; then
            echo "Found ${INDY_DIR}"
        elif [ -z "$2" ] ; then
            echo STDERR "Missing INDY_DIR argument and environment variable"
            echo STDERR "e.g. set INDY_DIR=<path> for environment or libindy_${TARGET_ARCH}"
            exit 1
        else
            INDY_DIR=$2
        fi
        if [ -d "${INDY_DIR}/lib" ] ; then
            INDY_DIR="${INDY_DIR}/lib"
        fi
 fi

source ${CI_DIR}/setup.android.env.sh
generate_arch_flags ${TARGET_ARCH}

echo ">> in runner script"
declare -a EXE_ARRAY

build_test_artifacts(){
    pushd ${WORKDIR}
        echo $LIBRARY_PATH
        echo $LD_LIBRARY_PATH
        cargo clean

        RUSTFLAGS="-L${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB} -lz -lm -L${TOOLCHAIN_DIR}/${TRIPLET}/lib  -lgnustl_shared" \
            cargo build --target=${TRIPLET} --release

       # Tests are failing to build for some reason. See IS-
       # RUSTFLAGS="-L${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB} -lz -lm -L${TOOLCHAIN_DIR}/${TRIPLET}/lib  -lgnustl_shared" \
       #             cargo test --target=${TRIPLET} --no-run --release

       # EXE_ARRAY=($( RUSTFLAGS="-L${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB} -lz -lm -L${TOOLCHAIN_DIR}/${TRIPLET}/lib  -lgnustl_shared" \
       #             cargo test --target=${TRIPLET} --no-run --release --message-format=json | jq -r "select(.profile.test == true) | .filenames[]"))

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

    adb -e push \
    "${LIBINDY_DIR}/libindy.so" "/data/local/tmp/libindy.so"

    adb -e push \
    "${LIBINDY_DIR}/libindy.a" "/data/local/tmp/libindy.a"

    for i in "${EXE_ARRAY[@]}"
    do
       :
        EXE="${i}"
        EXE_NAME=`basename ${EXE}`


        adb push "$EXE" "/data/local/tmp/$EXE_NAME"
        adb shell "chmod 755 /data/local/tmp/$EXE_NAME"
        OUT="$(mktemp)"
        MARK="ADB_SUCCESS!"
        adb shell "LD_LIBRARY_PATH=/data/local/tmp RUST_TEST_THREADS=1 RUST_LOG=debug /data/local/tmp/$EXE_NAME && echo $MARK" 2>&1 | tee $OUT
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
