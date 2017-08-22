FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && apt-get install openjdk-8-jdk -y

ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64

RUN apt-get install -y \
      maven \
      gdebi \
      apt-utils

ADD https://repo.evernym.com/libindy/ubuntu/master/0.1.1-119/indy-sdk_0.1.1_amd64.deb .

RUN gdebi -n indy-sdk_0.1.1_amd64.deb

RUN useradd -ms /bin/bash -u $uid indy
USER indy

WORKDIR /home/indy