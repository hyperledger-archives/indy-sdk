FROM ubuntu:16.04
LABEL maintainer="Mohammad Abdul Sami <faisal00813@gmail.com>"


RUN DEBIAN_FRONTEND=noninteractive apt-get -qq update -y && \
    apt-get -qq install -y \
    zip \
    unzip \
    libtool \
    curl \
    wget \
    python3 \
    pkg-config \
    libncursesw5-dev \
    build-essential \
    libzmq3-dev \
    jq 2>&1 > /dev/null

RUN useradd -m -d /home/indy_user -s /bin/bash indy_user && mkdir -p /etc/sudoers.d/
RUN echo "indy_user ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/indy_user
RUN chmod 0440 /etc/sudoers.d/indy_user
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.26.0
ENV PATH /home/indy/.cargo/bin:$PATH

USER indy_user
WORKDIR /home/indy_user

COPY android-ndk-r16b-linux-x86_64.zip /home/indy_user/
RUN unzip -qq /home/indy_user/android-ndk-r16b-linux-x86_64.zip -d /home/indy_user/
COPY --chown=indy_user:indy_user indy-sdk/ /home/indy_user/indy-sdk/
COPY --chown=indy_user:indy_user ${openssl_dir}/ /home/indy_user/openssl/
COPY --chown=indy_user:indy_user ${sodium_dir}/ /home/indy_user/sodium/
COPY --chown=indy_user:indy_user ${libzmq_dir}/ /home/indy_user/libzmq/
COPY --chown=indy_user:indy_user make_indy.sh /home/indy_user/
RUN chmod a+x make_indy.sh
RUN ./make_indy.sh
RUN echo "libindy android build successful"
