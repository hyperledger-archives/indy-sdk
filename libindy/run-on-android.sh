#!/bin/sh

set -e
EXE="$1"
EXE_NAME=`basename $EXE`

TARGET_ARCH="x86"
TARGET_API="21"
CROSS_COMPILE="i686-linux-android"
echo ">> in runner script"
WORKDIR=${PWD}

#Before pushing to the adb link all the dependencies of the indy statically
#---------------------------
## One alternative is to create the one big .a file with all the dependencies, let us call it dep.a
## then use the dep.a to link to executable. Ref: http://nickdesaulniers.github.io/blog/2016/11/20/static-and-dynamic-libraries/
#---------------------------
#Statically link the libraries with the binary
#${CC} -v -shared -o final_${EXE} -Wl,--whole-archive ${EXE} ${TOOLCHAIN_DIR}/sysroot/usr/lib/libz.so ${TOOLCHAIN_DIR}/sysroot/usr/lib/libm.a ${TOOLCHAIN_DIR}/sysroot/usr/lib/liblog.so ${OPENSSL_DIR}/lib/libssl.a ${OPENSSL_DIR}/lib/libcrypto.a ${SODIUM_LIB_DIR}/libsodium.a ${LIBZMQ_LIB_DIR}/libzmq.a ${TOOLCHAIN_DIR}/${CROSS_COMPILE}/lib/libstdc++.a -Wl,--no-whole-archive -z muldefs

~/Work/repos/faisal00813/indy-sdk/libindy/ci/sdk/platform-tools/adb push \
    "${HOME}/Work/repos/faisal00813/indy-sdk/libindy/build_scripts/android/toolchains/linux/x86/i686-linux-android/lib/libgnustl_shared.so" "/data/local/tmp/libgnustl_shared.so"
~/Work/repos/faisal00813/indy-sdk/libindy/ci/sdk/platform-tools/adb push "$EXE" "/data/local/tmp/$EXE_NAME"
~/Work/repos/faisal00813/indy-sdk/libindy/ci/sdk/platform-tools/adb shell "chmod 755 /data/local/tmp/$EXE_NAME"
OUT="$(mktemp)"
MARK="ADB_SUCCESS!"
~/Work/repos/faisal00813/indy-sdk/libindy/ci/sdk/platform-tools/adb shell "LD_LIBRARY_PATH=/data/local/tmp RUST_LOG=debug /data/local/tmp/$EXE_NAME && echo $MARK" 2>&1 | tee $OUT
grep $MARK $OUT