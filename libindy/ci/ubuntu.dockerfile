FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      libssl-dev \
      libgmp3-dev \
      curl \
      build-essential \
      autoconf \
      automake \
      libtool \
      libsqlite3-dev \
      cmake \
      git \
      python3.5 \
      python3-pip \
      python-setuptools \
      apt-transport-https \
      ca-certificates \
      debhelper \
      wget \
      devscripts \
      libncursesw5-dev \
      zip \
      unzip \
      jq

# install nodejs and npm
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash -
RUN apt-get install -y nodejs

RUN pip3 install -U \
	pip \
	setuptools \
	virtualenv \
	twine \
	plumbum \
	deb-pkg-tools

ARG LIBSODIUM_VERSION=1.0.17

RUN cd /tmp && \
   curl https://download.libsodium.org/libsodium/releases/libsodium-$LIBSODIUM_VERSION.tar.gz | tar -xz && \
    cd /tmp/libsodium-$LIBSODIUM_VERSION && \
    ./configure && \
    make && \
    make install && \
    rm -rf /tmp/libsodium-$LIBSODIUM_VERSION && \
    ldconfig -n /usr/local/lib

ARG LIBZEROMQ_VERSION=4.3.1

RUN cd /tmp && \
   curl https://codeload.github.com/zeromq/libzmq/tar.gz/v$LIBZEROMQ_VERSION | tar -xz && \
    cd /tmp/libzmq-$LIBZEROMQ_VERSION && \
    ./autogen.sh && ./configure && \
    make && \
    make install && \
    rm -rf /tmp/libzmq-$LIBZEROMQ_VERSION && \
    ldconfig -n /usr/local/lib

RUN apt-get update && apt-get install openjdk-8-jdk -y
ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64
RUN apt-get update && apt-get install -y maven

RUN apt-get install -y zip

RUN apt-get update && apt-get install -y --no-install-recommends \
        ruby \
        ruby-dev \
        rubygems \
    && gem install --no-ri --no-rdoc rake fpm \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash -u $uid indy
USER indy

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.36.0
ENV PATH /home/indy/.cargo/bin:$PATH

# Install clippy to the Rust toolchain
RUN rustup component add clippy

WORKDIR /home/indy
