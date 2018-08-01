# Development
FROM ubuntu:16.04

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
    software-properties-common \
    debhelper \
    wget \
    git \
    curl \
	libffi-dev \
    ruby \
    ruby-dev \ 
	sudo \
    rubygems \
    libzmq5 \
    python3

# Install Nodejs 
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash - \
    && apt-get install -y nodejs

# Install Rust
ARG RUST_VER="1.27.0"
ENV RUST_ARCHIVE=rust-${RUST_VER}-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fOL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

# fpm for deb packaging of npm
RUN gem install fpm
RUN apt-get install rpm -y

COPY ./vcx/ci/scripts/installCert.sh /tmp
RUN /tmp/installCert.sh

# Add sovrin to sources.list
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 && \
    add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial master" && \
    add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable" && \
    add-apt-repository 'deb https://repo.sovrin.org/deb xenial master' && \
    add-apt-repository 'deb https://repo.corp.evernym.com/deb evernym-agency-dev-ubuntu main' && \
    curl https://repo.corp.evernym.com/repo.corp.evenym.com-sig.key | apt-key add -

ARG LIBINDY_VER="1.6.1"
ARG LIBNULL_VER="1.6.1"
ARG LIBSOVTOKEN_VER="0.8.0+2.54"

RUN apt-get update && apt-get install -y \
    libsovtoken=${LIBSOVTOKEN_VER} \
    libindy=${LIBINDY_VER} \
