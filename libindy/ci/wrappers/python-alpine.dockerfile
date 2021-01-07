ARG WRAPPER_BASE_IMAGE
FROM ${WRAPPER_BASE_IMAGE}

USER root

# RUN apt-get update && \
#     apt-get install -y \
#       python3.5 \
#       python-setuptools

RUN apk update && apk upgrade && \
    apk add --no-cache \
       python3 \
       py3-pip \
       py3-setuptools

# RUN curl -fsSL -o- https://bootstrap.pypa.io/pip/get-pip.py | python3

RUN pip install -U \
	setuptools==47.0.0 \
	# virtualenv \
	twine==1.15.0 \
	plumbum==1.6.7 six==1.15.0 \
	deb-pkg-tools
