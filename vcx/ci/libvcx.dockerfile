FROM libindy
ARG uid=1000

RUN useradd -ms /bin/bash -u $uid vcx
USER vcx

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.26.0
ENV PATH /home/vcx/.cargo/bin:$PATH
WORKDIR /home/vcx
ENV PATH /home/vcx:$PATH
# cargo deb for debian packaging of libvcx
RUN cargo install cargo-deb --color=never




# RUN cargo update-version
# RUN cargo test --color=never --no-default-features --features "ci sovtoken" -- --test-threads=1
# RUN cargo update-so
# RUN cargo deb --no-build
# RUN find -type f -name "libvcx*.deb" -exec dpkg -i {} \;
