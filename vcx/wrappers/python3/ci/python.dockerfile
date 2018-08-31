FROM libindy
ARG uid=1000
RUN useradd -ms /bin/bash -u $uid python

RUN apt-get update && apt-get install -y python3

RUN apt-get install -y python3-pip

RUN pip3 install pytest==3.4.2 qrcode pytest-asyncio

ENV PYTHONPATH=${PYTHONPATH}:vcx/wrappers/python3

RUN find . -name \*.pyc -delete
COPY vcx/libvcx/target/debian/*.deb .
RUN dpkg -i *.deb
USER python
