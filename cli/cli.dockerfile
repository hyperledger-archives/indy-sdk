FROM ubuntu:16.04

RUN apt-get update && apt-get install -y apt-transport-https

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
RUN echo "deb https://repo.sovrin.org/sdk/deb xenial master" >> /etc/apt/sources.list

RUN apt-get update && apt-get install -y indy-cli
