FROM amazonlinux:2017.03

ARG uid=0

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
           spectool

ENV RUST_ARCHIVE=rust-1.16.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir -p /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
    && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
    && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
    && rm $RUST_ARCHIVE \
    && ./install.sh

ENV PATH="/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/root/.cargo/bin"

RUN useradd -m -G root -u $uid sovrin
RUN groups sovrin
USER sovrin

WORKDIR /home/sovrin