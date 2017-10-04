FROM amazonlinux:2017.03

ARG uid=1000

RUN \
    yum clean all \
    && yum upgrade -y \
    && yum groupinstall -y "Development Tools" \
    && yum install -y epel-release \
    && yum-config-manager --enable epel \
    && yum install -y \
           wget \
           cmake \
           pkgconfig \
           openssl-devel \
           sqlite-devel \
           libsodium-devel \
           spectool \
           python35 \
           python35-pip


RUN cd /tmp && \
   curl https://download.libsodium.org/libsodium/releases/libsodium-1.0.12.tar.gz | tar -xz && \
    cd /tmp/libsodium-1.0.12 && \
    ./configure && \
    make && \
    make install && \
    rm -rf /tmp/libsodium-1.0.12

ENV PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib

RUN yum install -y java-1.8.0-openjdk-devel
ENV JAVA_HOME /usr/lib/jvm/java-1.8.0-openjdk

RUN wget https://repos.fedorapeople.org/repos/dchen/apache-maven/epel-apache-maven.repo -O /etc/yum.repos.d/epel-apache-maven.repo
RUN sed -i s/\$releasever/6/g /etc/yum.repos.d/epel-apache-maven.repo
RUN yum install -y apache-maven

ENV RUST_ARCHIVE=rust-1.20.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin"

RUN useradd -ms /bin/bash -u $uid indy
USER indy

RUN cargo install --git https://github.com/DSRCorporation/cargo-test-xunit

WORKDIR /home/sorvin
