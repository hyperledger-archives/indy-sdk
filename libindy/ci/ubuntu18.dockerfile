FROM ubuntu:18.04

ARG uid=1000

RUN apt-get update && \
    apt-get install -y \
      pkg-config \
      libssl-dev \
      curl \
      libsqlite3-dev \
      cmake \
      python3-pip \
      debhelper \
      devscripts \
      libncursesw5-dev \
      libzmq3-dev \
      libsodium-dev

RUN pip3 install -U \
	pip \
	twine \
	plumbum \
	deb-pkg-tools

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

WORKDIR /home/indy