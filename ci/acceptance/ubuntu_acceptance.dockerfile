FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y \
    openjdk-8-jdk \
    maven

ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64

RUN apt-get update && \
      apt-get install -y \
      software-properties-common

RUN apt-get update && \
      apt-get install -y \
      python3.5 \
      python3-pip \
      vim

RUN pip3 install --upgrade pip
RUN pip3 install -U pip

ARG indy_sdk_deb
RUN echo ${indy_sdk_deb}
ADD ${indy_sdk_deb} indy-sdk.deb

RUN dpkg -i indy-sdk.deb || apt-get install -y -f

RUN useradd -ms /bin/bash indy
USER indy
WORKDIR /home/indy
