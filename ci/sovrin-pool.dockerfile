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

RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys EAA542E8
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys D82D8E35
RUN echo "deb https://repo.evernym.com/deb xenial master" >> /etc/apt/sources.list
RUN echo "deb https://repo.sovrin.org/deb xenial master" >> /etc/apt/sources.list

RUN useradd -ms /bin/bash -u $uid sovrin

RUN apt-get update -y && apt-get install -y \
    sovrin-node

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
user = sovrin\n\
identifier = supervisor\n\
directory = /tmp\n\
nocleanup = true\n\
childlogdir = /tmp\n\
strip_ansi = false\n\
\n\
[program:node1]\n\
command=start_sovrin_node Node1 9701 9702\n\
directory=/home/sovrin\n\
stdout_logfile=/tmp/node1.log\n\
stderr_logfile=/tmp/node1.log\n\
loglevel=trace\n\
\n\
[program:node2]\n\
command=start_sovrin_node Node2 9703 9704\n\
directory=/home/sovrin\n\
stdout_logfile=/tmp/node2.log\n\
stderr_logfile=/tmp/node2.log\n\
loglevel=trace\n\
\n\
[program:node3]\n\
command=start_sovrin_node Node3 9705 9706\n\
directory=/home/sovrin\n\
stdout_logfile=/tmp/node3.log\n\
stderr_logfile=/tmp/node3.log\n\
loglevel=trace\n\
\n\
[program:node4]\n\
command=start_sovrin_node Node4 9707 9708\n\
directory=/home/sovrin\n\
stdout_logfile=/tmp/node4.log\n\
stderr_logfile=/tmp/node4.log\n\
loglevel=trace\n'\
>> /etc/supervisord.conf

USER sovrin

RUN init_sovrin_keys --name Node1 --seed 111111111111111111111111111Node1 --force
RUN init_sovrin_keys --name Node2 --seed 111111111111111111111111111Node2 --force
RUN init_sovrin_keys --name Node3 --seed 111111111111111111111111111Node3 --force
RUN init_sovrin_keys --name Node4 --seed 111111111111111111111111111Node4 --force

RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 1 --ips="10.0.0.2,10.0.0.2,10.0.0.2,10.0.0.2"
RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 2 --ips="10.0.0.2,10.0.0.2,10.0.0.2,10.0.0.2"
RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 3 --ips="10.0.0.2,10.0.0.2,10.0.0.2,10.0.0.2"
RUN generate_sovrin_pool_transactions --nodes 4 --clients 5 --nodeNum 4 --ips="10.0.0.2,10.0.0.2,10.0.0.2,10.0.0.2"

EXPOSE 9701 9702 9703 9704 9705 9706 9707 9708 9709

CMD ["/usr/bin/supervisord"]
