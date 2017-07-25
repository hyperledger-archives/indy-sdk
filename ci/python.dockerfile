FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && \
      apt-get install -y \
      gdebi \
      apt-utils \
      software-properties-common

RUN add-apt-repository ppa:jonathonf/python-3.6

RUN apt-get update && \
      apt-get install -y \
      python3.6 \
      python3-pip

ADD https://repo.evernym.com/deb/indy-sdk/0.1.1/indy-sdk_0.1.1_amd64.deb .

RUN gdebi -n indy-sdk_0.1.1_amd64.deb

RUN useradd -ms /bin/bash -u $uid indy
USER indy

WORKDIR /home/indy



