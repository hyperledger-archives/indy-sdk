ARG WRAPPER_BASE_IMAGE
FROM ${WRAPPER_BASE_IMAGE}

RUN apt-get update && \
    apt-get install -y \
      python3.5 \
      python-setuptools

RUN curl -fsSL -o- https://bootstrap.pypa.io/pip/3.6/get-pip.py  | python3.6

RUN pip install -U \
	setuptools \
	virtualenv \
	twine==1.15.0 \
	plumbum==1.6.7 six==1.12.0 \
	deb-pkg-tools
