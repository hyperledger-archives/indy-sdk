#!/usr/bin/env bash

set -e
set -o pipefail
WORKDIR=${PWD}
LIBINDY_WORKDIR=${WORKDIR}
CI_DIR="${LIBINDY_WORKDIR}/ci"
export ANDROID_BUILD_FOLDER="/tmp/android_build"
DOWNLOAD_PREBUILTS="0"

while getopts ":d" opt; do
    case ${opt} in
        d) export DOWNLOAD_PREBUILTS="1";;
        \?);;
    esac
done
shift $((OPTIND -1))

TARGET_ARCH=$1

if [ -z "${TARGET_ARCH}" ]; then
    echo STDERR "Missing TARGET_ARCH argument"
    echo STDERR "e.g. x86 or arm"
    exit 1
fi

source ${CI_DIR}/setup.android.env.sh

create_cargo_config(){
mkdir -p ${LIBINDY_WORKDIR}/.cargo
cat << EOF > ${LIBINDY_WORKDIR}/.cargo/config
[target.${TRIPLET}]
ar = "$(realpath ${AR})"
linker = "$(realpath ${CC})"
EOF
}
setup_dependencies(){
    if [ "${DOWNLOAD_PREBUILTS}" == "1" ]; then
        download_and_unzip_dependencies_for_all_architectures
        else
            echo "not downloading prebuilt dependencies. Dependencies locations have to be passed"
            if [ -z "${OPENSSL_DIR}" ]; then
                OPENSSL_DIR="openssl_${TARGET_ARCH}"
                if [ -d "${OPENSSL_DIR}" ] ; then
                    echo "Found ${OPENSSL_DIR}"
                elif [ -z "$4" ]; then
                    echo STDERR "Missing OPENSSL_DIR argument and environment variable"
                    echo STDERR "e.g. set OPENSSL_DIR=<path> for environment or openssl_${TARGET_ARCH}"
                    exit 1
                else
                    OPENSSL_DIR=$4
                fi
            fi

            if [ -z "${SODIUM_DIR}" ]; then
                SODIUM_DIR="libsodium_${TARGET_ARCH}"
                if [ -d "${SODIUM_DIR}" ] ; then
                    echo "Found ${SODIUM_DIR}"
                elif [ -z "$5" ]; then
                    echo STDERR "Missing SODIUM_DIR argument and environment variable"
                    echo STDERR "e.g. set SODIUM_DIR=<path> for environment or libsodium_${TARGET_ARCH}"
                    exit 1
                else
                    SODIUM_DIR=$5
                fi
            fi

            if [ -z "${LIBZMQ_DIR}" ] ; then
                LIBZMQ_DIR="libzmq_${TARGET_ARCH}"
                if [ -d "${LIBZMQ_DIR}" ] ; then
                    echo "Found ${LIBZMQ_DIR}"
                elif [ -z "$6" ] ; then
                    echo STDERR "Missing LIBZMQ_DIR argument and environment variable"
                    echo STDERR "e.g. set LIBZMQ_DIR=<path> for environment or libzmq_${TARGET_ARCH}"
                    exit 1
                else
                    LIBZMQ_DIR=$6
                fi
            fi


    fi
}

statically_link_dependencies_with_libindy(){
    $CC -v -shared -o${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib/libindy.so -Wl,--whole-archive \
        ${WORKDIR}/target/${TRIPLET}/release/libindy.a \
        ${TOOLCHAIN_DIR}/${TOOLCHAIN_SYSROOT}/libz.so \
        ${TOOLCHAIN_DIR}/${TOOLCHAIN_SYSROOT}/libm.a \
        ${TOOLCHAIN_DIR}/${TOOLCHAIN_SYSROOT}/liblog.so \
        ${OPENSSL_DIR}/lib/libssl.a \
        ${OPENSSL_DIR}/lib/libcrypto.a \
        ${SODIUM_LIB_DIR}/libsodium.a \
        ${LIBZMQ_LIB_DIR}/libzmq.a \
        ${TOOLCHAIN_DIR}/${ANDROID_TRIPLET}/lib/libgnustl_shared.so \
        -Wl,--no-whole-archive -z muldefs
}

package_library(){

    mkdir -p ${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib

    cp -rf "${WORKDIR}/include" ${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}
    cp "${WORKDIR}/target/${TRIPLET}/release/libindy.a" ${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib
    cp "${WORKDIR}/target/${TRIPLET}/release/libindy.so" ${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib
    mv "${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib/libindy.so" "${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}/lib/libindy_shared.so" &&
    statically_link_dependencies_with_libindy &&
    pushd ${LIBINDY_WORKDIR}
        rm -f libindy_android_${TARGET_ARCH}.zip
        cp -rf ${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH} .
        if [ -z "${LIBINDY_VERSION}" ]; then
            zip -r libindy_android_${TARGET_ARCH}.zip libindy_${TARGET_ARCH}
        else
            zip -r libindy_android_${TARGET_ARCH}_${LIBINDY_VERSION}.zip libindy_${TARGET_ARCH}
        fi

    popd
}

build(){
    echo "**************************************************"
    echo "Building for architecture ${TARGET_ARCH}"
    echo "Toolchain path ${TOOLCHAIN_DIR}"
    echo "ZMQ path ${LIBZMQ_DIR}"
    echo "Sodium path ${SODIUM_DIR}"
    echo "Openssl path ${OPENSSL_DIR}"
    echo "Artifacts will be in ${ANDROID_BUILD_FOLDER}/libindy_${TARGET_ARCH}"
    echo "**************************************************"
    pushd ${WORKDIR}
        rm -rf target/${TRIPLET}
        cargo clean &&
        LD_LIBRARY_PATH=${TOOLCHAIN_DIR}/${TOOLCHAIN_SYSROOT} \
        RUSTFLAGS="-C link-args=-Wl,-rpath,${TOOLCHAIN_DIR}/${TOOLCHAIN_SYSROOT} -L${TOOLCHAIN_DIR}/${ANDROID_TRIPLET}/lib -lgnustl_shared" \
        cargo build --release --verbose --target=${TRIPLET} &&
    popd
}


generate_arch_flags ${TARGET_ARCH}
setup_dependencies
download_and_setup_toolchain
set_env_vars
create_standalone_toolchain_and_rust_target
create_cargo_config
build && package_library