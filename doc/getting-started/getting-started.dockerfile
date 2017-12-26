FROM jupyter/base-notebook:latest

USER root

RUN apt-get update \
    && apt-get install -y \
    software-properties-common \
    apt-transport-https

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 \
    && add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial master" \
    && apt-get update \
    && apt-get install -y \
    libindy

USER $NB_USER

RUN pip install python3-indy