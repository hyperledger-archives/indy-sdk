FROM ubuntu:16.04

ARG uid=1000

# Install environment
RUN apt-get update -y && apt-get install -y \
	git \
	wget \
	python3.5 \
	python3-pip \
	python-setuptools \
	python3-nacl \
	apt-transport-https \
	ca-certificates \
	supervisor

RUN pip3 install -U \
	pip==9.0.3 \
	setuptools

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
ARG indy_stream=rc
RUN echo "deb https://repo.sovrin.org/deb xenial $indy_stream" >> /etc/apt/sources.list

RUN useradd -ms /bin/bash -u $uid indy

ARG indy_plenum_ver=1.2.38
ARG indy_anoncreds_ver=1.0.11
ARG indy_node_ver=1.3.56
ARG python3_indy_crypto_ver=0.2.0
ARG indy_crypto_ver=0.1.6

RUN apt-get update -y && apt-get install -y \
        indy-plenum=${indy_plenum_ver} \
        indy-anoncreds=${indy_anoncreds_ver} \
        indy-node=${indy_node_ver} \
        python3-indy-crypto=${python3_indy_crypto_ver} \
        libindy-crypto=${indy_crypto_ver} \
        vim

RUN echo '[supervisord]\n\
logfile = /tmp/supervisord.log\n\
logfile_maxbytes = 50MB\n\
logfile_backups=10\n\
logLevel = error\n\
pidfile = /tmp/supervisord.pid\n\
nodaemon = true\n\
minfds = 1024\n\
minprocs = 200\n\
umask = 022\n\
user = indy\n\
identifier = supervisor\n\
directory = /tmp\n\
nocleanup = true\n\
childlogdir = /tmp\n\
strip_ansi = false\n\
\n\
[program:node1]\n\
command=start_indy_node Node1 9701 9702\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node1.log\n\
stderr_logfile=/tmp/node1.log\n\
\n\
[program:node2]\n\
command=start_indy_node Node2 9703 9704\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node2.log\n\
stderr_logfile=/tmp/node2.log\n\
\n\
[program:node3]\n\
command=start_indy_node Node3 9705 9706\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node3.log\n\
stderr_logfile=/tmp/node3.log\n\
\n\
[program:node4]\n\
command=start_indy_node Node4 9707 9708\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node4.log\n\
stderr_logfile=/tmp/node4.log\n'\
>> /etc/supervisord.conf

USER indy

ARG pool_ip=127.0.0.1

RUN awk '{if (index($1, "NETWORK_NAME") != 0) {print("NETWORK_NAME = \"sandbox\"")} else print($0)}' /etc/indy/indy_config.py> /tmp/indy_config.py
RUN mv /tmp/indy_config.py /etc/indy/indy_config.py

RUN generate_indy_pool_transactions --nodes 4 --clients 5 --nodeNum 1 2 3 4 --ips="$pool_ip,$pool_ip,$pool_ip,$pool_ip"

EXPOSE 9701 9702 9703 9704 9705 9706 9707 9708

CMD ["/usr/bin/supervisord"]
