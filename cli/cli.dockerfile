FROM ubuntu:20.04

RUN DEBIAN_FRONTEND=noninteractive 

RUN apt-get update -y && TZ=Etc/UTC apt-get -y install tzdata

RUN apt-get update -y && apt-get install -y \
    software-properties-common \
    apt-transport-https \
    curl \
    build-essential \
    git \
    libzmq3-dev \
    libsodium-dev \
    pkg-config \
    libncurses5 \
    libssl-dev \
    gnupg \
    ca-certificates


RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88
RUN add-apt-repository "deb https://repo.sovrin.org/sdk/deb bionic stable"
RUN apt-get update -y && apt-get install -y --allow-unauthenticated \
    libindy \
    indy-cli