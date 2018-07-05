FROM ubuntu:16.04
LABEL maintainer="Mohammad Abdul Sami <faisal00813@gmail.com>"

ARG target_arch
ARG target_api
ARG cross_compile
ARG indy_dir

ENV TARGET_ARCH ${target_arch}
ENV TARGET_API ${target_api}
ENV CROSS_COMPILE ${cross_compile}
ENV INDY_DIR /home/indy_user/indy
ENV ANDROID_NDK_ROOT /home/indy_user/android-ndk-r16b
ENV TOOLCHAIN_DIR /home/indy_user/${target_arch}
ENV PATH ${TOOLCHAIN_DIR}/bin:${PATH}
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV CC=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang
ENV AR=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ar
ENV CXX=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-clang++
ENV CXXLD=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ld
ENV RANLIB=${TOOLCHAIN_DIR}/bin/${CROSS_COMPILE}-ranlib
ENV TARGET=android

RUN DEBIAN_FRONTEND=noninteractive apt-get -qq update -y && apt-get -qq install -y zip unzip libtool curl wget python3 pkg-config libzmq3-dev 2>&1 > /dev/null
RUN useradd -m -d /home/indy_user -s /bin/bash indy_user && mkdir -p /etc/sudoers.d/
RUN echo "indy_user ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/indy_user
RUN chmod 0440 /etc/sudoers.d/indy_user

USER indy_user
WORKDIR /home/indy_user

COPY android-ndk-r16b-linux-x86_64.zip /home/indy_user/
RUN unzip -qq /home/indy_user/android-ndk-r16b-linux-x86_64.zip -d /home/indy_user/
COPY --chown=indy_user:indy_user indy-sdk/ /home/indy_user/indy-sdk/
COPY --chown=indy_user:indy_user ${indy_dir}/ /home/indy_user/indy/
COPY --chown=indy_user:indy_user make_nullpay.sh /home/indy_user/
RUN chmod a+x make_nullpay.sh
RUN ./make_nullpay.sh
RUN echo "libnullpay android build successful"
