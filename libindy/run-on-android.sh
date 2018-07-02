#!/bin/sh

set -e
EXE="$1"
EXE_NAME=`basename $EXE`
#Before pushing to the adb link all the dependencies of the indy statically
#---------------------------
## One alternative is to create the one big .a file with all the dependencies, let us call it dep.a
## then use the dep.a to link to executable. Ref: http://nickdesaulniers.github.io/blog/2016/11/20/static-and-dynamic-libraries/
#---------------------------
#Statically link the libraries with the binary
#${i686_LINUX_ANDROID_LINKER} -v -shared -o final_${EXE} -Wl,--whole-archive ${EXE} ${TOOLCHAIN_DIR}/sysroot/usr/lib/libz.so ${TOOLCHAIN_DIR}/sysroot/usr/lib/libm.a ${TOOLCHAIN_DIR}/sysroot/usr/lib/liblog.so ${OPENSSL_DIR}/lib/libssl.a ${OPENSSL_DIR}/lib/libcrypto.a ${SODIUM_LIB_DIR}/libsodium.a ${LIBZMQ_LIB_DIR}/libzmq.a ${TOOLCHAIN_DIR}/${CROSS_COMPILE}/lib/libstdc++.a -Wl,--no-whole-archive -z muldefs

adb push "$EXE" "/data/local/tmp/$EXE_NAME"
adb shell "chmod 755 /data/local/tmp/$EXE_NAME"
OUT="$(mktemp)"
MARK="ADB_SUCCESS!"
adb shell "RUST_LOG=debug /data/local/tmp/$EXE_NAME && echo $MARK" 2>&1 | tee $OUT
grep $MARK $OUT