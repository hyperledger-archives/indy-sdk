# Development
FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      libssl-dev \
      libgmp3-dev \
      curl \
      build-essential \
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
      libzmq3-dev \



# Indy USER
RUN useradd -ms /bin/bash -u $uid indy
RUN usermod -aG sudo indy


USER indy
# cargo deb for debian packaging of libindy
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

