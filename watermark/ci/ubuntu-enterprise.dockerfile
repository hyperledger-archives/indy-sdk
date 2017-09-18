# Development
FROM ubuntu:16.04

ARG uid=1000

# Update environment
# JRE installation
RUN apt-get update -y && apt-get install -y default-jre

# fakeroot installation
RUN apt-get install -y fakeroot

# libsodium installation
RUN apt-get install -y libsodium18

# Install curl
RUN apt-get update && apt-get install -y curl

# Install sbt
RUN apt-get install -y apt-transport-https
RUN echo "deb https://dl.bintray.com/sbt/debian /" | tee -a /etc/apt/sources.list.d/sbt.list
RUN apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv 2EE0EA64E40A89B84B2DF73499E82A75642AC823
RUN apt-get update && apt-get install -y sbt=0.13.13

RUN useradd -ms /bin/bash -u $uid sovrin
USER sovrin
WORKDIR /home/sovrin
RUN echo "==> Fetch all sbt jars from Maven(?) repo..."     && \
    echo "==> [CAUTION] this may take several minutes."     && \
    sbt sbtVersion
VOLUME /home/sovrin
