FROM ubuntu:16.04

# Install necessary libraries
RUN apt-get update && \
      apt-get install -y curl \
      apt-transport-https \
      software-properties-common \
      build-essential \
      git \
      pkg-config \
      cmake \
      libssl-dev \
      libsqlite3-dev \
      libzmq3-dev \
      libncursesw5-dev

# LibIndy requires `libsodium 1.0.14`, but `apt` doesn't support it. Gotta build & install from source
RUN cd /tmp && \
      curl https://download.libsodium.org/libsodium/releases/old/libsodium-1.0.14.tar.gz | tar -xz && \
      cd /tmp/libsodium-1.0.14 && \
      ./configure --disable-shared && \
      make && \
      make install && \
      rm -rf /tmp/libsodium-1.0.14

# Install indy-cli from binary
RUN apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88 && \
    add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable" && \
    apt-get update && \
    apt-get install -y indy-cli

# Download Rust
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_8.x | bash && \
    apt-get install nodejs -y

RUN mkdir indy-sdk/
WORKDIR indy-sdk/
ADD . .
EXPOSE 4000
CMD [ "/bin/bash" ]