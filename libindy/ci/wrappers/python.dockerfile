ARG WRAPPER_BASE_IMAGE
FROM ${WRAPPER_BASE_IMAGE}

USER root

RUN apt-get update && \
    apt-get install -y \
      python3.5 \
      python3-pip \
      python-setuptools

RUN pip3 install -U \
	pip \
	setuptools \
	virtualenv \
	twine==1.15.0 \
	plumbum==1.6.7 six==1.12.0 \
	deb-pkg-tools

USER indy
