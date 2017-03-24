FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      libzmq3-dev \
      libssl-dev \
      curl \
      build-essential \
      sqlite3 \
      libsqlite3-dev

ENV RUST_ARCHIVE=rust-1.16.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

RUN cargo install --git https://github.com/DSRCorporation/cargo-test-xunit

ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin"

RUN mkdir -p /home/sorvin-client-rust
WORKDIR /home/sorvin-client-rust