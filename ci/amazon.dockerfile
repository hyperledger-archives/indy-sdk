FROM amazonlinux:2017.03

ARG uid=1000

RUN \
    yum clean all \
    && yum install -y epel-release \
    && yum upgrade -y \
    && yum groupinstall -y "Development Tools" \
    && yum install -y \
           cmake \
           pkgconfig \
           openssl-devel \
           sqlite-devel \
           wget

RUN cd /tmp && \
    curl https://download.libsodium.org/libsodium/releases/libsodium-1.0.12.tar.gz | tar -xz && \
    cd /tmp/libsodium-1.0.12 && \
    ./configure && \
    make && \
    make install && \
    rm -rf /tmp/libsodium-1.0.12

ENV PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib

RUN cd /tmp && \
    wget https://github.com/zeromq/libzmq/releases/download/v4.2.0/zeromq-4.2.0.tar.gz && \
    tar xfz zeromq-4.2.0.tar.gz && rm zeromq-4.2.0.tar.gz && \
    cd /tmp/zeromq-4.2.0 && \
    ./configure && \
    make && \
    make install && \
    rm -rf /tmp/zeromq-4.2.0

ENV RUST_ARCHIVE=rust-1.16.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin"

RUN useradd -ms /bin/bash -u $uid sovrin
USER sovrin

WORKDIR /home/sovrin