#!/usr/bin/env bash


WORKDIR=${PWD}
INDY_WORKDIR="$(realpath "${WORKDIR}/..")"
CI_DIR="$(realpath "${WORKDIR}/../ci")"
BUILD_FOLDER="$(realpath "${WORKDIR}/../android_build")"
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

    if [ -z "${INDY_DIR}" ]; then
    echo STDERR "Missing INDY_DIR argument"
    echo STDERR "Should have path to directory containing libindy binaries"
    echo "Sample : ./build.withoutdocker.sh x86 16 i686-linux-android <ABSOLUTE_PATH_TO_LIBINDY_BINARIES_DIR>"
    exit 1
fi


}


package_library(){
    mkdir -p ${BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/include
    mkdir -p ${BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/lib

    cp "${WORKDIR}/target/${TRIPLET}/release/libnullpay.a" ${BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/lib
    cp "${WORKDIR}/target/${TRIPLET}/release/libnullpay.so" ${BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/lib
}

build(){
    pushd ${WORKDIR}
        rm -rf target/${TRIPLET}
        cargo clean
        cargo build --release --target=${TRIPLET}
    popd
}


#cleanup(){
##    rm -rf ${BUILD_FOLDER}
#
#}

#execute_build_steps(){
#
#        test
##        build
##        package_library
#}

generate_arch_flags ${TARGET_ARCH}
setup_dependencies
#download_and_unzip_dependencies_for_all_architectures
download_and_setup_toolchain
set_env_vars
create_standalone_toolchain_and_rust_target
printenv
build
package_library