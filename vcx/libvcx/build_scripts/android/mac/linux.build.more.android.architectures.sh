#!/bin/bash


declare -a archs=(
    "arm" "arm" "16" "arm-linux-androideabi"
    "arm" "armv7" "16" "arm-linux-androideabi"
    "arm64" "arm64" "21" "aarch64-linux-android"
    "x86" "x86" "16" "i686-linux-android"
    "x86_64" "x86_64" "21" "x86_64-linux-android"
    )
archslen=${#archs[@]}

# !IMPORTANT: The architecture at index 0 is already built by the first invocation of the libsodium Dockerfile
# that is why this for loop starts at index 4
for (( arch=4; arch<${archslen}; arch=arch+4 ));
do
    #echo $arch " / " ${archslen} " : " ${archs[$arch]}
    export ndk_arch=${archs[$arch]}
    export target_arch=${archs[$arch+1]}
    export target_api=${archs[$arch+2]}
    export cross_compile=${archs[$arch+3]}
    export TARGET_ARCH=${target_arch}
    export TARGET_API=${target_api}
    export CROSS_COMPILE=${cross_compile}
    export TOOLCHAIN_DIR=/home/sodium_user/${ndk_arch}
    export PATH=${TOOLCHAIN_DIR}/bin:${PATH}
    export CC=${TOOLCHAIN_DIR}/bin/${cross_compile}-clang
    export AR=${TOOLCHAIN_DIR}/bin/${cross_compile}-ar
    export CXX=${TOOLCHAIN_DIR}/bin/${cross_compile}-clang++
    export CXXLD=${TOOLCHAIN_DIR}/bin/${cross_compile}-ld
    export RANLIB=${TOOLCHAIN_DIR}/bin/${cross_compile}-ranlib
    cd /home/sodium_user
    echo "Building Android NDK for architecture ${target_arch}"
    python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${ndk_arch} --api ${target_api} --install-dir ${TOOLCHAIN_DIR}
    cd /home/sodium_user/libsodium-1.0.12
    make clean
    ./autogen.sh
    ./configure --prefix=/home/sodium_user/libsodium_${TARGET_ARCH} --disable-soname-versions --host=${CROSS_COMPILE}
    make
    make install
    cd /home/sodium_user
    zip libsodium_${TARGET_ARCH}.zip -r libsodium_${TARGET_ARCH}
    cp libsodium_${TARGET_ARCH}.zip /data/libsodium
    echo "libsodium android build for ${target_arch} successful"
done


cd /home/sodium_user
if [ ! -f "zeromq-4.2.5.tar.gz" ] ; then
    echo "Downloading zeromq-4.2.5.tar.gz"
    wget -q https://github.com/zeromq/libzmq/releases/download/v4.2.5/zeromq-4.2.5.tar.gz
    tar xf /home/sodium_user/zeromq-4.2.5.tar.gz -C /home/sodium_user/
else
    echo "Skipping download zeromq-4.2.5.tar.gz"
fi

for (( arch=0; arch<${archslen}; arch=arch+4 ));
do
    export ndk_arch=${archs[$arch]}
    export target_arch=${archs[$arch+1]}
    export target_api=${archs[$arch+2]}
    export cross_compile=${archs[$arch+3]}
    export TARGET_ARCH=${target_arch}
    export sodium_lib_dir=/home/sodium_user/libsodium_${TARGET_ARCH}/lib
    export TARGET_API=${target_api}
    export CROSS_COMPILE=${cross_compile}
    export ZMQ_HAVE_ANDROID=1
    export SODIUM_LIB_DIR=${sodium_lib_dir}
    export TOOLCHAIN_DIR=/home/sodium_user/${ndk_arch}
    export PATH=${TOOLCHAIN_DIR}/bin:${PATH}
    python3 ${ANDROID_NDK_ROOT}/build/tools/make_standalone_toolchain.py --arch ${ndk_arch} --api ${target_api} --install-dir ${TOOLCHAIN_DIR}
    cd /home/sodium_user/zeromq-4.2.5
    make clean
    ./autogen.sh
    ./configure CPP=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-cpp CC=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang \
    CXX=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang++ LD=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ld \
    AS=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-as AR=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ar \
    RANLIB=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ranlib CFLAGS="-I/home/sodium_user/libzmq_${TARGET_ARCH}/include -D__ANDROID_API__=${TARGET_API} -fPIC" \
    CXXFLAGS="-I/home/sodium_user/libzmq_${TARGET_ARCH}/include -D__ANDROID_API__=${TARGET_API} -fPIC" \
    LDFLAGS="-L/home/sodium_user/libzmq_${TARGET_ARCH}/lib -D__ANDROID_API__=${TARGET_API}" LIBS="-lc -lgcc -ldl" \
    --host=${CROSS_COMPILE} --prefix=/home/sodium_user/libzmq_${TARGET_ARCH} --with-libsodium=${SODIUM_LIB_DIR} \
    --without-docs --enable-static --with-sysroot=${TOOLCHAIN_DIR}/sysroot
    make
    make install
    cd /home/sodium_user
    rm -rf libzmq_${TARGET_ARCH}/bin
    zip libzmq_${TARGET_ARCH}.zip -r libzmq_${TARGET_ARCH}
    cp libzmq_${TARGET_ARCH}.zip /data/zmq
    echo "libzmq android build for ${target_arch} successful"
done
