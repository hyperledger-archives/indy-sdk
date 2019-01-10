FROM ubuntu:16.04

ARG uid=1000

RUN apt-get update && \
    apt-get install -y


RUN useradd -m -u $uid indy
USER indy


# Install Rust toolchain (rustc, cargo, rustup, etc.)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.31.0
ENV PATH /home/indy/.cargo/bin:$PATH

# Install clippy to the Rust toolchain
RUN rustup component add clippy-preview

WORKDIR /home/indy
