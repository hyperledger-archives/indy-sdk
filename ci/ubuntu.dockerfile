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
    libsqlite0 \
    cmake \
    apt-transport-https \
    ca-certificates \
    debhelper \
    wget \
    git \
    curl \
	libffi-dev \
    ruby \
    ruby-dev \ 
	sudo \
    rubygems \
    libzmq5

# Install Nodejs 
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash - \
    && apt-get install -y nodejs

RUN npm install typescript-compiler

# Install libindy
RUN mkdir -p /libindy
WORKDIR /libindy

ENV LIBINDY_DEB=libindy_1.4.0~605_amd64.deb
ENV LIBINDY_DOWNLOAD_URL=https://repo.sovrin.org/sdk/lib/apt/xenial/master/$LIBINDY_DEB

RUN curl -fsOSL $LIBINDY_DOWNLOAD_URL \
    && dpkg -i $LIBINDY_DEB \
    && apt-get -f install

# Install libnullpay
ENV LIBNULLPAY_DEB=libnullpay_0.1.0~605_amd64.deb
ENV LIBNULLPAY_DOWNLOAD_URL=https://repo.sovrin.org/sdk/lib/apt/xenial/master/$LIBNULLPAY_DEB

RUN curl -fsOSL $LIBNULLPAY_DOWNLOAD_URL\
    && dpkg -i $LIBNULLPAY_DEB\
    && apt-get -f install

# Install Rust
ENV RUST_ARCHIVE=rust-1.25.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

# fpm for deb packaging of npm
RUN gem install fpm
RUN apt-get install rpm -y

RUN useradd -ms /bin/bash -u $uid vcx
USER vcx

# cargo deb for debian packaging of libvcx
RUN cargo install cargo-deb --color=never



