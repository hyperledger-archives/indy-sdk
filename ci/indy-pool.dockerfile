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
	software-properties-common \
	supervisor

RUN pip3 install -U \
	"pip~=9.0" \
	"setuptools~=50.0"

RUN add-apt-repository "deb http://us.archive.ubuntu.com/ubuntu xenial main universe" && \
	apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys CE7709D068DB5E88
ARG indy_stream=master
RUN add-apt-repository "deb https://repo.sovrin.org/deb xenial ${indy_stream}" && \
	add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable"

RUN useradd -ms /bin/bash -u $uid indy

ARG indy_plenum_ver=1.13.0.dev1032
ARG indy_node_ver=1.13.0.dev1221

RUN apt-get update -y && apt-get install -y \
	libsodium18 \
	libbz2-dev \
	zlib1g-dev \
	liblz4-dev \
	libsnappy-dev \
	rocksdb=5.8.8 \
	libindy \
	ursa \
	vim

RUN pip3 install \
	indy-plenum==${indy_plenum_ver} \
	indy-node==${indy_node_ver}

RUN echo "[supervisord]\n\
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
command=start_indy_node Node1 0.0.0.0 9701 0.0.0.0 9702\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node1.log\n\
stderr_logfile=/tmp/node1.log\n\
\n\
[program:node2]\n\
command=start_indy_node Node2 0.0.0.0 9703 0.0.0.0 9704\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node2.log\n\
stderr_logfile=/tmp/node2.log\n\
\n\
[program:node3]\n\
command=start_indy_node Node3 0.0.0.0 9705 0.0.0.0 9706\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node3.log\n\
stderr_logfile=/tmp/node3.log\n\
\n\
[program:node4]\n\
command=start_indy_node Node4 0.0.0.0 9707 0.0.0.0 9708\n\
directory=/home/indy\n\
stdout_logfile=/tmp/node4.log\n\
stderr_logfile=/tmp/node4.log\n"\
>> /etc/supervisord.conf

RUN mkdir -p \
	/etc/indy \
	/var/lib/indy/backup \
	/var/lib/indy/plugins \
	/var/log/indy \
	&& chown -R indy:root /etc/indy /var/lib/indy /var/log/indy

USER indy

RUN echo "LEDGER_DIR = '/var/lib/indy'\n\
LOG_DIR = '/var/log/indy'\n\
KEYS_DIR = '/var/lib/indy'\n\
GENESIS_DIR = '/var/lib/indy'\n\
BACKUP_DIR = '/var/lib/indy/backup'\n\
PLUGINS_DIR = '/var/lib/indy/plugins'\n\
NODE_INFO_DIR = '/var/lib/indy'\n\
NETWORK_NAME = 'sandbox'\n"\
>> /etc/indy/indy_config.py

ARG pool_ip=127.0.0.1

RUN generate_indy_pool_transactions --nodes 4 --clients 5 --nodeNum 1 2 3 4 --ips="$pool_ip,$pool_ip,$pool_ip,$pool_ip"

EXPOSE 9701 9702 9703 9704 9705 9706 9707 9708

CMD ["/usr/bin/supervisord"]
