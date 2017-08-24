FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && \
      apt-get install -y \
      gdebi \
      apt-utils \
      software-properties-common \
      ruby-dev \
      build-essential \
      git

RUN add-apt-repository ppa:jonathonf/python-3.6

RUN apt-get update && \
      apt-get install -y \
      python3.6 \
      python3-pip

ADD https://repo.evernym.com/libindy/ubuntu/master/0.1.1-132/indy-sdk_0.1.1_amd64.deb .

RUN gdebi -n indy-sdk_0.1.1_amd64.deb

RUN gem install fpm

ADD https://bootstrap.pypa.io/ez_setup.py .
RUN python3.6

RUN python3.6 -m pip install twine

RUN useradd -ms /bin/bash -u $uid indy
USER indy

WORKDIR /home/indy



