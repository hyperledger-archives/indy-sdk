FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      libzmq3-dev \
      libssl-dev \
      curl \
      build-essential \
      libsqlite3-dev \
      libsodium-dev \
      cmake

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

RUN cargo install --git https://github.com/DSRCorporation/cargo-test-xunit

WORKDIR /home/sovrin