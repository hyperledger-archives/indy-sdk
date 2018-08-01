#!/usr/bin/env bash


WORKDIR="$( cd "$(dirname "$0")" ; pwd -P )"
CI_DIR="${WORKDIR}/../libindy/ci"
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
mkdir -p ${WORKDIR}/.cargo
cat << EOF > ${WORKDIR}/.cargo/config
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
                elif [ -z "$3" ]; then
                    echo STDERR "Missing OPENSSL_DIR argument and environment variable"
                    echo STDERR "e.g. set OPENSSL_DIR=<path> for environment or openssl_${TARGET_ARCH}"
                    exit 1
                else
                    OPENSSL_DIR=$3
                fi
            fi

            if [ -z "${SODIUM_DIR}" ]; then
                SODIUM_DIR="libsodium_${TARGET_ARCH}"
                if [ -d "${SODIUM_DIR}" ] ; then
                    echo "Found ${SODIUM_DIR}"
                elif [ -z "$4" ]; then
                    echo STDERR "Missing SODIUM_DIR argument and environment variable"
                    echo STDERR "e.g. set SODIUM_DIR=<path> for environment or libsodium_${TARGET_ARCH}"
                    exit 1
                else
                    SODIUM_DIR=$4
                fi
            fi

            if [ -z "${LIBZMQ_DIR}" ] ; then
                LIBZMQ_DIR="libzmq_${TARGET_ARCH}"
                if [ -d "${LIBZMQ_DIR}" ] ; then
                    echo "Found ${LIBZMQ_DIR}"
                elif [ -z "$5" ] ; then
                    echo STDERR "Missing LIBZMQ_DIR argument and environment variable"
                    echo STDERR "e.g. set LIBZMQ_DIR=<path> for environment or libzmq_${TARGET_ARCH}"
                    exit 1
                else
                    LIBZMQ_DIR=$5
                fi
            fi


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


}


package_library(){
    mkdir -p ${ANDROID_BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/include
    mkdir -p ${ANDROID_BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/lib

    cp "${WORKDIR}/target/${TRIPLET}/release/libnullpay.a" ${ANDROID_BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/lib
    cp "${WORKDIR}/target/${TRIPLET}/release/libnullpay.so" ${ANDROID_BUILD_FOLDER}/libnullpay_${TARGET_ARCH}/lib

     pushd ${WORKDIR}
        rm -f libnullpay_android_${TARGET_ARCH}.zip
        cp -rf ${ANDROID_BUILD_FOLDER}/libnullpay_${TARGET_ARCH} .
        if [ -z "${LIBNULLPAY_VERSION}" ]; then
            zip -r libnullpay_android_${TARGET_ARCH}.zip libnullpay_${TARGET_ARCH}
        else
            zip -r libnullpay_android_${TARGET_ARCH}_${LIBNULLPAY_VERSION}.zip libnullpay_${TARGET_ARCH}
        fi

    popd
}

build(){
    echo "**************************************************"
    echo "Building for architecture ${TARGET_ARCH}"
    echo "Toolchain path ${TOOLCHAIN_DIR}"
    echo "ZMQ path ${LIBZMQ_DIR}"
    echo "Sodium path ${SODIUM_DIR}"
    echo "Indy path ${INDY_DIR}"
    echo "Artifacts will be in ${ANDROID_BUILD_FOLDER}/libnullpay_${TARGET_ARCH}"
    echo "**************************************************"
    pushd ${WORKDIR}
        rm -rf target/${TRIPLET}
        cargo clean
        cargo build --release --target=${TRIPLET}
    popd
}


generate_arch_flags ${TARGET_ARCH}
setup_dependencies
download_and_unzip_dependencies_for_all_architectures
download_and_setup_toolchain
set_env_vars
create_standalone_toolchain_and_rust_target
create_cargo_config
build
package_library