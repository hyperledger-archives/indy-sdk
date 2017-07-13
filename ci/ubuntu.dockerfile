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
      cmake \
      git \
      python3.5 \
      python3-pip

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys BD33704C
RUN echo "deb https://repo.evernym.com/deb xenial master" >> /etc/apt/sources.list
RUN apt-get update -y
RUN apt-get install -y \
	python3-charm-crypto

RUN pip3 install -U \
	pip \
	setuptools \
	virtualenv \
	git+https://github.com/hyperledger/indy-anoncreds.git

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

RUN useradd -ms /bin/bash -u $uid indy
USER indy

RUN cargo install --git https://github.com/DSRCorporation/cargo-test-xunit

WORKDIR /home/indy

RUN git clone https://github.com/hyperledger/indy-anoncreds.git
RUN virtualenv -p python3.5 /home/indy/
RUN cp -r /usr/local/lib/python3.5/dist-packages/Charm_Crypto-0.0.0.egg-info /home/indy/lib/python3.5/site-packages/Charm_Crypto-0.0.0.egg-info
RUN cp -r /usr/local/lib/python3.5/dist-packages/charm /home/indy/lib/python3.5/site-packages/charm
USER root
RUN ln -sf /home/indy/bin/python /usr/local/bin/python
RUN ln -sf /home/indy/bin/pip /usr/local/bin/pip
USER indy