FROM ubuntu:18.04

ARG RUST_VER=1.58.0

RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      curl \
      cmake \
      build-essential \
      libssl-dev \
      libzmq3-dev \
      python3-distutils

RUN cd /tmp && \
   curl https://download.libsodium.org/libsodium/releases/libsodium-1.0.18.tar.gz | tar -xz && \
    cd /tmp/libsodium-1.0.18 && \
    ./configure && \
    make && \
    make install && \
    ldconfig && \
    rm -rf /tmp/libsodium-1.0.18

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VER}
ENV PATH /root/.cargo/bin:$PATH

RUN cargo install cargo-deb cargo-bump

WORKDIR /root

COPY libindy indy-sdk/libindy
COPY wrappers indy-sdk/wrappers
ARG INDYSDK_PATH=/root/indy-sdk

RUN cargo build --release --manifest-path=$INDYSDK_PATH/libindy/Cargo.toml
RUN mv $INDYSDK_PATH/libindy/target/release/libindy.so /usr/lib

RUN rm -rf indy-sdk
