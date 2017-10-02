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
	pip \
	setuptools

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
ARG indy_stream=master
RUN echo "deb https://repo.sovrin.org/deb xenial $indy_stream" >> /etc/apt/sources.list

RUN useradd -ms /bin/bash -u $uid indy

ARG indy_plenum_ver=1.1.136
ARG indy_anoncreds_ver=1.0.25
ARG indy_node_ver=1.1.149
ARG python3_indy_crypto_ver=0.1.6
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
loglevel = trace\n\
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
command=start_sovrin_node Node1 9701 9702\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node1.log\n\
stderr_logfile=/tmp/node1.log\n\
loglevel=trace\n\
\n\
[program:node2]\n\
command=start_sovrin_node Node2 9703 9704\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node2.log\n\
stderr_logfile=/tmp/node2.log\n\
loglevel=trace\n\
\n\
[program:node3]\n\
command=start_sovrin_node Node3 9705 9706\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node3.log\n\
stderr_logfile=/tmp/node3.log\n\
loglevel=trace\n\
\n\
[program:node4]\n\
command=start_sovrin_node Node4 9707 9708\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node4.log\n\
stderr_logfile=/tmp/node4.log\n\
loglevel=trace\n'\
>> /etc/supervisord.conf

USER indy

RUN init_sovrin_keys --name Node1 --seed 111111111111111111111111111Node1 --force
RUN init_sovrin_keys --name Node2 --seed 111111111111111111111111111Node2 --force
RUN init_sovrin_keys --name Node3 --seed 111111111111111111111111111Node3 --force
RUN init_sovrin_keys --name Node4 --seed 111111111111111111111111111Node4 --force

ARG pool_ip=127.0.0.1

RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 1 --ips="$pool_ip,$pool_ip,$pool_ip,$pool_ip"
RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 2 --ips="$pool_ip,$pool_ip,$pool_ip,$pool_ip"
RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 3 --ips="$pool_ip,$pool_ip,$pool_ip,$pool_ip"
RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 4 --ips="$pool_ip,$pool_ip,$pool_ip,$pool_ip"

EXPOSE 9701 9702 9703 9704 9705 9706 9707 9708 9709

CMD ["/usr/bin/supervisord"]
