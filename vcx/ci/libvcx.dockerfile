FROM libindy
ARG uid=1000

RUN useradd -ms /bin/bash -u $uid vcx
USER vcx

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.34.1
ENV PATH /home/vcx/.cargo/bin:$PATH
WORKDIR /home/vcx
ENV PATH /home/vcx:$PATH
# cargo deb for debian packaging of libvcx
RUN cargo install cargo-deb --color=never
