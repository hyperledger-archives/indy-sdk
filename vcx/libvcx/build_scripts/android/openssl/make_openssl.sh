#!/bin/bash
# Cross-compile environment for Android on ARMv7 and x86
#
# Contents licensed under the terms of the OpenSSL license
# http://www.openssl.org/source/license.html
#
# See http://wiki.openssl.org/index.php/FIPS_Library_and_Android
#   and http://wiki.openssl.org/index.php/Android

#####################################################################

# Set ANDROID_NDK_ROOT to you NDK location. For example,
# /opt/android-ndk-r8e or /opt/android-ndk-r9. This can be done in a
# login script. If ANDROID_NDK_ROOT is not specified, the script will
# try to pick it up with the value of _ANDROID_NDK_ROOT below. If
# ANDROID_NDK_ROOT is set, then the value is ignored.
# _ANDROID_NDK="android-ndk-r8e"
_ANDROID_NDK="android-ndk-r20"
# _ANDROID_NDK="android-ndk-r10"

# Set _ANDROID_EABI to the EABI you want to use. You can find the
# list in $ANDROID_NDK_ROOT/toolchains. This value is always used.
# _ANDROID_EABI="x86-4.6"
# _ANDROID_EABI="arm-linux-androideabi-4.6"
_ANDROID_EABI="${TARGET_ARCH}-4.9"

# Set _ANDROID_ARCH to the architecture you are building for.
# This value is always used.
# _ANDROID_ARCH=arch-x86
_ANDROID_ARCH=arch-${TARGET_ARCH}

# Set _ANDROID_API to the API you want to use. You should set it
# to one of: android-14, android-9, android-8, android-14, android-5
# android-4, or android-3. You can't set it to the latest (for
# example, API-17) because the NDK does not supply the platform. At
# Android 5.0, there will likely be another platform added (android-22?).
# This value is always used.
# _ANDROID_API="android-14"
_ANDROID_API="android-${TARGET_API}"
# _ANDROID_API="android-19"

#####################################################################

# If the user did not specify the NDK location, try and pick it up.
# We expect something like ANDROID_NDK_ROOT=/opt/android-ndk-r8e
# or ANDROID_NDK_ROOT=/usr/local/android-ndk-r8e.

if [ -z "$ANDROID_NDK_ROOT" ]; then

  _ANDROID_NDK_ROOT=""
  if [ -z "$_ANDROID_NDK_ROOT" ] && [ -d "/usr/local/$_ANDROID_NDK" ]; then
    _ANDROID_NDK_ROOT="/usr/local/$_ANDROID_NDK"
  fi

  if [ -z "$_ANDROID_NDK_ROOT" ] && [ -d "/opt/$_ANDROID_NDK" ]; then
    _ANDROID_NDK_ROOT="/opt/$_ANDROID_NDK"
  fi

  if [ -z "$_ANDROID_NDK_ROOT" ] && [ -d "$HOME/$_ANDROID_NDK" ]; then
    _ANDROID_NDK_ROOT="$HOME/$_ANDROID_NDK"
  fi

  if [ -z "$_ANDROID_NDK_ROOT" ] && [ -d "$PWD/$_ANDROID_NDK" ]; then
    _ANDROID_NDK_ROOT="$PWD/$_ANDROID_NDK"
  fi

  # If a path was set, then export it
  if [ ! -z "$_ANDROID_NDK_ROOT" ] && [ -d "$_ANDROID_NDK_ROOT" ]; then
    export ANDROID_NDK_ROOT="$_ANDROID_NDK_ROOT"
  fi
fi

# Error checking
# ANDROID_NDK_ROOT should always be set by the user (even when not running this script)
# http://groups.google.com/group/android-ndk/browse_thread/thread/a998e139aca71d77
if [ -z "$ANDROID_NDK_ROOT" ] || [ ! -d "$ANDROID_NDK_ROOT" ]; then
  echo "Error: ANDROID_NDK_ROOT is not a valid path. Please edit this script."
  # echo "$ANDROID_NDK_ROOT"
  # exit 1
fi

# Error checking
if [ ! -d "$ANDROID_NDK_ROOT/toolchains" ]; then
  echo "Error: ANDROID_NDK_ROOT/toolchains is not a valid path. Please edit this script."
  # echo "$ANDROID_NDK_ROOT/toolchains"
  # exit 1
fi

# Error checking
if [ ! -d "$ANDROID_NDK_ROOT/toolchains/$_ANDROID_EABI" ]; then
  echo "Error: ANDROID_EABI is not a valid path. Please edit this script."
  # echo "$ANDROID_NDK_ROOT/toolchains/$_ANDROID_EABI"
  # exit 1
fi

#####################################################################

# Based on ANDROID_NDK_ROOT, try and pick up the required toolchain. We expect something like:
# /opt/android-ndk-r83/toolchains/arm-linux-androideabi-4.7/prebuilt/linux-x86_64/bin
# Once we locate the toolchain, we add it to the PATH. Note: this is the 'hard way' of
# doing things according to the NDK documentation for Ice Cream Sandwich.
# https://android.googlesource.com/platform/ndk/+/ics-mr0/docs/STANDALONE-TOOLCHAIN.html

ANDROID_TOOLCHAIN=""
for host in "linux-x86_64" "linux-x86" "darwin-x86_64" "darwin-x86"
do
  if [ -d "$ANDROID_NDK_ROOT/toolchains/$_ANDROID_EABI/prebuilt/$host/bin" ]; then
    ANDROID_TOOLCHAIN="$ANDROID_NDK_ROOT/toolchains/$_ANDROID_EABI/prebuilt/$host/bin"
    break
  fi
done

# Error checking
if [ -z "$ANDROID_TOOLCHAIN" ] || [ ! -d "$ANDROID_TOOLCHAIN" ]; then
  echo "Error: ANDROID_TOOLCHAIN is not valid. Please edit this script."
  # echo "$ANDROID_TOOLCHAIN"
  # exit 1
fi

case $_ANDROID_ARCH in
	arch-arm)
      ANDROID_TOOLS="arm-linux-androideabi-clang arm-linux-androideabi-ranlib arm-linux-androideabi-ld"
	  ;;
	arch-x86)
      ANDROID_TOOLS="i686-linux-android-clang i686-linux-android-ranlib i686-linux-android-ld"
	  ;;
	*)
	  echo "ERROR ERROR ERROR"
	  ;;
esac

for tool in $ANDROID_TOOLS
do
  # Error checking
  if [ ! -e "$ANDROID_TOOLCHAIN/$tool" ]; then
    echo "Error: Failed to find $tool. Please edit this script."
    # echo "$ANDROID_TOOLCHAIN/$tool"
    # exit 1
  fi
done

# Only modify/export PATH if ANDROID_TOOLCHAIN good
if [ ! -z "$ANDROID_TOOLCHAIN" ]; then
  export ANDROID_TOOLCHAIN="$ANDROID_TOOLCHAIN"
  export PATH="$ANDROID_TOOLCHAIN":"$PATH"
fi

#####################################################################

# For the Android SYSROOT. Can be used on the command line with --sysroot
# https://android.googlesource.com/platform/ndk/+/ics-mr0/docs/STANDALONE-TOOLCHAIN.html
export ANDROID_SYSROOT="$TOOLCHAIN_DIR/sysroot"
export CROSS_SYSROOT="$ANDROID_SYSROOT"
export NDK_SYSROOT="$ANDROID_SYSROOT"

# Error checking
if [ -z "$ANDROID_SYSROOT" ] || [ ! -d "$ANDROID_SYSROOT" ]; then
  echo "Error: ANDROID_SYSROOT is not valid. Please edit this script."
  # echo "$ANDROID_SYSROOT"
  # exit 1
fi

#####################################################################

# If the user did not specify the FIPS_SIG location, try and pick it up
# If the user specified a bad location, then try and pick it up too.
if [ -z "$FIPS_SIG" ] || [ ! -e "$FIPS_SIG" ]; then

  # Try and locate it
  _FIPS_SIG=""
  if [ -d "/usr/local/ssl/$_ANDROID_API" ]; then
    _FIPS_SIG=`find "/usr/local/ssl/$_ANDROID_API" -name incore`
  fi

  if [ ! -e "$_FIPS_SIG" ]; then
    _FIPS_SIG=`find $PWD -name incore`
  fi

  # If a path was set, then export it
  if [ ! -z "$_FIPS_SIG" ] && [ -e "$_FIPS_SIG" ]; then
    export FIPS_SIG="$_FIPS_SIG"
  fi
fi

# Error checking. Its OK to ignore this if you are *not* building for FIPS
if [ -z "$FIPS_SIG" ] || [ ! -e "$FIPS_SIG" ]; then
  echo "Error: FIPS_SIG does not specify incore module. Please edit this script."
  # echo "$FIPS_SIG"
  # exit 1
fi

#####################################################################

# Most of these should be OK (MACHINE, SYSTEM, ARCH). RELEASE is ignored.
export MACHINE=armv7
export RELEASE=2.6.37
export SYSTEM=android
export ARCH=arm
export CROSS_COMPILE="arm-linux-androideabi-"

if [ "$_ANDROID_ARCH" == "arch-x86" ]; then
	export MACHINE=i686
	export RELEASE=2.6.37
	export SYSTEM=android
	export ARCH=x86
	export CROSS_COMPILE="i686-linux-android-"
fi

# For the Android toolchain
# https://android.googlesource.com/platform/ndk/+/ics-mr0/docs/STANDALONE-TOOLCHAIN.html
export ANDROID_SYSROOT="$TOOLCHAIN_DIR/sysroot"
export SYSROOT="$ANDROID_SYSROOT"
export NDK_SYSROOT="$ANDROID_SYSROOT"
export ANDROID_NDK_SYSROOT="$ANDROID_SYSROOT"
export ANDROID_API="$_ANDROID_API"

# CROSS_COMPILE and ANDROID_DEV are DFW (Don't Fiddle With). Its used by OpenSSL build system.
# export CROSS_COMPILE="arm-linux-androideabi-"
export ANDROID_DEV="$ANDROID_NDK_ROOT/platforms/$_ANDROID_API/$_ANDROID_ARCH/usr"
export HOSTCC=clang

VERBOSE=1
if [ ! -z "$VERBOSE" ] && [ "$VERBOSE" != "0" ]; then
  echo "ANDROID_NDK_ROOT: $ANDROID_NDK_ROOT"
  echo "ANDROID_ARCH: $_ANDROID_ARCH"
  echo "ANDROID_EABI: $_ANDROID_EABI"
  echo "ANDROID_API: $ANDROID_API"
  echo "ANDROID_SYSROOT: $ANDROID_SYSROOT"
  echo "ANDROID_TOOLCHAIN: $ANDROID_TOOLCHAIN"
  echo "FIPS_SIG: $FIPS_SIG"
  echo "CROSS_COMPILE: $CROSS_COMPILE"
  echo "ANDROID_DEV: $ANDROID_DEV"
fi

cd openssl-1.1.0h
./config -D__ANDROID_API__=${TARGET_ARCH} --openssldir=${HOME}/openssl_${TARGET_ARCH} --prefix=${HOME}/openssl_${TARGET_ARCH} -lc -lgcc -ldl
make
make install
rm -rf ${HOME}/openssl_${TARGET_ARCH}/share
rm -rf ${HOME}/openssl_${TARGET_ARCH}/misc
rm -rf ${HOME}/openssl_${TARGET_ARCH}/certs
rm -rf ${HOME}/openssl_${TARGET_ARCH}/private
rm -rf ${HOME}/openssl_${TARGET_ARCH}/bin
rm -f ${HOME}/openssl_${TARGET_ARCH}/openssl.cnf
rm -f ${HOME}/openssl_${TARGET_ARCH}/openssl.cnf.dist
cd ${HOME}
zip openssl_${TARGET_ARCH}.zip -r openssl_${TARGET_ARCH}
echo "openssl android build successful"
