# TODO: Try alpine for wrapper images
FROM alpine:3.12

ARG INDYSDK_PATH=/home/indy/indy-sdk
ARG RUST_VER=1.46.0
ENV RUST_LOG=warning

RUN apk update && apk upgrade && \
    apk add --no-cache \
        build-base \
        cargo \
        libsodium-dev \
        libzmq \
        openssl-dev \
        zeromq-dev \
        bash \
        curl

RUN cargo install cargo-bump

WORKDIR /root

COPY --chown=indy:indy libindy indy-sdk/libindy
COPY --chown=indy:indy wrappers indy-sdk/wrappers
# COPY --chown=indy:indy experimental indy-sdk/experimental
# COPY --chown=indy:indy libnullpay indy-sdk/libnullpay

ARG INDYSDK_PATH=/root/indy-sdk

RUN cargo build --release --manifest-path=$INDYSDK_PATH/libindy/Cargo.toml

# USER root
RUN mv $INDYSDK_PATH/libindy/target/release/libindy.so /usr/lib

# USER indy
# RUN cargo build --release --manifest-path=$INDYSDK_PATH/libnullpay/Cargo.toml
# RUN cargo build --release --manifest-path=$INDYSDK_PATH/experimental/plugins/postgres_storage/Cargo.toml

# USER root
# RUN mv $INDYSDK_PATH/libnullpay/target/release/libnullpay.so /usr/lib
# RUN mv $INDYSDK_PATH/experimental/plugins/postgres_storage/target/release/libindystrgpostgres.so /usr/lib

RUN rm -rf indy-sdk
 
# USER indy
