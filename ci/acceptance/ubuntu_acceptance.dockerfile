FROM ubuntu:16.04

ENV RUST_ARCHIVE=rust-1.19.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN apt-get update && apt-get install -y curl

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE

RUN ./install.sh

ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin"

RUN apt-get update && apt-get install -y openjdk-8-jdk

ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64

RUN apt-get update && apt-get install -y maven

RUN useradd -ms /bin/bash indy

RUN apt-get update && \
      apt-get install -y \
      software-properties-common

RUN add-apt-repository ppa:jonathonf/python-3.6
RUN apt-get update && \
      apt-get install -y \
      python3.6 \
      python3-pip

ARG indy_sdk_deb
RUN echo ${indy_sdk_deb}
ADD ${indy_sdk_deb} indy-sdk.deb

RUN dpkg -i indy-sdk.deb || apt-get install -y -f

WORKDIR /home/indy
USER indy
