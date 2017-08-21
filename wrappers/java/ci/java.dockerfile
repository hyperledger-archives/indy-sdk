FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && apt-get install openjdk-8-jdk -y

ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64

RUN apt-get install -y \
      maven \
      gdebi \
      apt-utils

RUN useradd -ms /bin/bash -u $uid indy
USER indy

WORKDIR /home/indy