#!/usr/bin/env bash
export BLACK=`tput setaf 0`
export RED=`tput setaf 1`
export GREEN=`tput setaf 2`
export YELLOW=`tput setaf 3`
export BLUE=`tput setaf 4`
export MAGENTA=`tput setaf 5`
export CYAN=`tput setaf 6`
export WHITE=`tput setaf 7`

export BOLD=`tput bold`
export RESET=`tput sgr0`

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
    echo STDERR "${RED}Missing TARGET_ARCH argument${RESET}"
    echo STDERR "${BLUE}e.g. x86 or arm${RESET}"
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
        download_and_unzip_dependencies ${ABSOLUTE_ARCH}
        else
            echo "${BLUE}Not downloading prebuilt dependencies. Dependencies locations have to be passed${RESET}"
            if [ -z "${OPENSSL_DIR}" ]; then
                OPENSSL_DIR="openssl_${ABSOLUTE_ARCH}"
                if [ -d "${OPENSSL_DIR}" ] ; then
                    echo "${GREEN}Found ${OPENSSL_DIR}${RESET}"
                elif [ -z "$2" ]; then
                    echo STDERR "${RED}Missing OPENSSL_DIR argument and environment variable${RESET}"
                    echo STDERR "${BLUE}e.g. set OPENSSL_DIR=<path> for environment or openssl_${ABSOLUTE_ARCH}${RESET}"
                    exit 1
                else
                    OPENSSL_DIR=$2
                fi
            fi

            if [ -z "${SODIUM_DIR}" ]; then
                SODIUM_DIR="libsodium_${ABSOLUTE_ARCH}"
                if [ -d "${SODIUM_DIR}" ] ; then
                    echo "${GREEN}Found ${SODIUM_DIR}${RESET}"
                elif [ -z "$3" ]; then
                    echo STDERR "${RED}Missing SODIUM_DIR argument and environment variable${RESET}"
                    echo STDERR "${BLUE}e.g. set SODIUM_DIR=<path> for environment or libsodium_${ABSOLUTE_ARCH}${RESET}"
                    exit 1
                else
                    SODIUM_DIR=$3
                fi
            fi

            if [ -z "${LIBZMQ_DIR}" ] ; then
                LIBZMQ_DIR="libzmq_${ABSOLUTE_ARCH}"
                if [ -d "${LIBZMQ_DIR}" ] ; then
                    echo "${GREEN}Found ${LIBZMQ_DIR}${RESET}"
                elif [ -z "$4" ] ; then
                    echo STDERR "${RED}Missing LIBZMQ_DIR argument and environment variable${RESET}"
                    echo STDERR "${BLUE}e.g. set LIBZMQ_DIR=<path> for environment or libzmq_${ABSOLUTE_ARCH}${RESET}"
                    exit 1
                else
                    LIBZMQ_DIR=$4
                fi
            fi


    fi
}

statically_link_dependencies_with_libindy(){
    echo "${BLUE}Statically linking libraries togather${RESET}"
    echo "${BLUE}Output will be available at ${ANDROID_BUILD_FOLDER}/libindy_${ABSOLUTE_ARCH}/lib/libindy.so${RESET}"
    $CC -v -shared -o${ANDROID_BUILD_FOLDER}/libindy_${ABSOLUTE_ARCH}/lib/libindy.so -Wl,--whole-archive \
        ${WORKDIR}/target/${TRIPLET}/release/libindy.a \
        ${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB}/libz.so \
        ${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB}/libm.a \
        ${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB}/liblog.so \
        ${OPENSSL_DIR}/lib/libssl.a \
        ${OPENSSL_DIR}/lib/libcrypto.a \
        ${SODIUM_LIB_DIR}/libsodium.a \
        ${LIBZMQ_LIB_DIR}/libzmq.a \
        ${TOOLCHAIN_DIR}/${ANDROID_TRIPLET}/${TOOLCHAIN_SYSROOT_LIB}/libgnustl_shared.so \
        -Wl,--no-whole-archive -z muldefs
}

package_library(){

   export PACKAGE_DIR=${ANDROID_BUILD_FOLDER}/libindy_${ABSOLUTE_ARCH}

    mkdir -p ${PACKAGE_DIR}/lib

    cp -rf "${WORKDIR}/include" ${PACKAGE_DIR}
    cp "${WORKDIR}/target/${TRIPLET}/release/libindy.a" ${PACKAGE_DIR}/lib
    cp "${WORKDIR}/target/${TRIPLET}/release/libindy.so" ${PACKAGE_DIR}/lib
    if [ "${TARGET_ARCH}" != "x86_64" ]; then
        mv "${PACKAGE_DIR}/lib/libindy.so" "${PACKAGE_DIR}/lib/libindy_shared.so" &&
        statically_link_dependencies_with_libindy
    fi
    pushd ${LIBINDY_WORKDIR}
        rm -f libindy_android_${ABSOLUTE_ARCH}.zip
        cp -rf ${PACKAGE_DIR} .
        if [ -z "${LIBINDY_VERSION}" ]; then
            zip -r libindy_android_${ABSOLUTE_ARCH}.zip libindy_${ABSOLUTE_ARCH} &&
            echo "${BLUE}Zip file available at ${PWD}/libindy_android_${ABSOLUTE_ARCH}.zip ${RESET}"
        else
            zip -r libindy_android_${ABSOLUTE_ARCH}_${LIBINDY_VERSION}.zip libindy_${ABSOLUTE_ARCH} &&
            echo "${BLUE}Zip file available at ${PWD}/libindy_android_${ABSOLUTE_ARCH}_${LIBINDY_VERSION}.zip ${RESET}"
        fi

    popd
}

build(){
    echo "**************************************************"
    echo "Building for architecture ${BOLD}${YELLOW}${ABSOLUTE_ARCH}${RESET}"
    echo "Toolchain path ${BOLD}${YELLOW}${TOOLCHAIN_DIR}${RESET}"
    echo "ZMQ path ${BOLD}${YELLOW}${LIBZMQ_DIR}${RESET}"
    echo "Sodium path ${BOLD}${YELLOW}${SODIUM_DIR}${RESET}"
    echo "Openssl path ${BOLD}${YELLOW}${OPENSSL_DIR}${RESET}"
    echo "Artifacts will be in ${YELLOW}${GREEN}${ANDROID_BUILD_FOLDER}/libindy_${ABSOLUTE_ARCH}${RESET}"
    echo "**************************************************"
    pushd ${WORKDIR}
        rm -rf target/${TRIPLET}
        cargo clean
        LD_LIBRARY_PATH=${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB} \
        RUSTFLAGS="-C link-args=-Wl,-rpath,${TOOLCHAIN_DIR}/sysroot/usr/${TOOLCHAIN_SYSROOT_LIB} -L${TOOLCHAIN_DIR}/${ANDROID_TRIPLET}/${TOOLCHAIN_SYSROOT_LIB} -lgnustl_shared" \
        cargo build --release --target=${TRIPLET}

    popd
}


generate_arch_flags ${TARGET_ARCH}
setup_dependencies
download_and_setup_toolchain
set_env_vars
create_standalone_toolchain_and_rust_target
create_cargo_config
build && package_library