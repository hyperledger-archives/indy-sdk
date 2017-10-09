# Development
FROM ubuntu:16.04

ARG uid=1000


# Update environment
# JRE installation and gcc
RUN apt-get update -y && apt-get install -y \
    gcc \
    pkg-config \
    build-essential \
    libsodium-dev \
    libssl-dev \
    libgmp3-dev \
    build-essential \
    libsqlite3-dev \
    cmake \
    apt-transport-https \
    ca-certificates \
    debhelper \
    wget \
    git \
    curl


# Install Nodejs 
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash - \
    && apt-get install -y nodejs

RUN npm install typescript-compiler


# Install Rust
ENV RUST_ARCHIVE=rust-1.20.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

RUN useradd -ms /bin/bash -u $uid cxs
USER cxs

