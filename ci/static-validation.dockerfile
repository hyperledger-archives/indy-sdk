FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && apt-get upgrade -y

RUN apt-get install -y \
        curl \
        git



RUN useradd -m -u $uid indy
USER indy


# Install Rust toolchain (rustc, cargo, rustup, etc.)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.31.0
ENV PATH /home/indy/.cargo/bin:$PATH

# Install clippy to the Rust toolchain
RUN rustup component add clippy

WORKDIR /home/indy
