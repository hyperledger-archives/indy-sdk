# Development
FROM ubuntu:16.04

ARG uid=1000


# Update environment
# JRE installation and gcc
RUN apt-get update -y && apt-get install -y default-jre gcc pkg-config build-essential git

# libsodium installation
#RUN apt-get install -y libsodium18

# Install curl
RUN apt-get update && apt-get install -y curl

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

#Sovrin stuff
RUN useradd -ms /bin/bash -u $uid sovrin
USER sovrin
#VOLUME /home/sovrin
