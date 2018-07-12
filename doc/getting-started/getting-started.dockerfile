FROM ubuntu:16.04

RUN useradd -ms /bin/bash indy

# Install environment
RUN apt-get update -y && apt-get install -y \
	wget \
	python3.5 \
	python3-pip \
	python-setuptools \
	ipython \
	ipython-notebook \
	apt-transport-https \
	ca-certificates \
	software-properties-common

WORKDIR /home/indy

RUN pip3 install -U \
	pip \
	setuptools \
	jupyter \
	python3-indy==1.5.0

<<<<<<< HEAD
RUN wget -O- "http://keyserver.ubuntu.com/pks/lookup?op=get&search=0x68DB5E88" | apt-key add -\
    && add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial master" \
=======
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 \
    && add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable" \
>>>>>>> upstream/master
    && apt-get update \
    && apt-get install -y \
    libindy=1.5.0

USER indy

EXPOSE 8888
