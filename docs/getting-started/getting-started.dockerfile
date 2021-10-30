FROM ubuntu:20.04

RUN useradd -ms /bin/bash indy

# Install environment
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Europe/Moscow
RUN apt-get update -y && apt-get install -y \
    wget \
    python3.5 \
    python3-pip \
    python-setuptools \
    apt-transport-https \
    ca-certificates \
    software-properties-common

WORKDIR /home/indy

RUN pip3 install -U \
    pip \
    ipython==7.9 \
    setuptools \
    jupyter \
    python3-indy==1.11.0

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88 \
    && add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable" \
    && add-apt-repository "deb http://security.ubuntu.com/ubuntu xenial-security main" \
    && apt-get update \
    && apt-get install -y \
    libssl1.0.0 libindy=1.11.0

USER indy

EXPOSE 8888
