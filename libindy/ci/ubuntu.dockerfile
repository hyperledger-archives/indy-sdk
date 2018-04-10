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
      libsodium-dev \
      cmake \
      git \
      python3.5 \
      python3-pip \
      python-setuptools \
      apt-transport-https \
      ca-certificates \
      debhelper \
      wget

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
RUN echo "deb https://repo.sovrin.org/deb xenial master" >> /etc/apt/sources.list
RUN apt-get update -y && apt-get install -y \
	python3-charm-crypto

RUN pip3 install -U \
	pip \
	setuptools \
	virtualenv

ENV RUST_ARCHIVE=rust-1.21.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin"

RUN apt-get update && apt-get install openjdk-8-jdk -y

ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64

RUN apt-get update && apt-get install -y maven

RUN useradd -ms /bin/bash -u $uid indy
USER indy

RUN cargo install --git https://github.com/DSRCorporation/cargo-test-xunit

WORKDIR /home/indy

USER root
RUN pip3 install \
    twine

RUN apt-get install -y devscripts \
                       libncursesw5-dev

RUN apt-get install -y libzmq3-dev

ARG anoncreds_revision=1.0.32-master
USER indy
RUN git clone https://github.com/hyperledger/indy-anoncreds.git
RUN cd indy-anoncreds && git checkout $anoncreds_revision
RUN virtualenv -p python3.5 /home/indy/test
RUN cp -r /usr/local/lib/python3.5/dist-packages/Charm_Crypto-0.0.0.egg-info /home/indy/test/lib/python3.5/site-packages/Charm_Crypto-0.0.0.egg-info
RUN cp -r /usr/local/lib/python3.5/dist-packages/charm /home/indy/test/lib/python3.5/site-packages/charm
USER root
RUN ln -sf /home/indy/test/bin/python /usr/local/bin/python3
RUN ln -sf /home/indy/test/bin/pip /usr/local/bin/pip3
USER indy
RUN pip3 install \
	/home/indy/indy-anoncreds \
	pytest

RUN pip3 install -U pip plumbum deb-pkg-tools