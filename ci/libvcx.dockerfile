FROM libindy
#
# cargo deb for debian packaging of libvcx
RUN cargo install cargo-deb --color=never

COPY . /sdk

# where debian will be copied to
RUN mkdir -p /sdk/vcx/output

ARG test_flag
ENV VCX_TEST_FLAG $test_flag
ENV PATH=${PATH}:/sdk/vcx/ci/scripts

WORKDIR /sdk/vcx/libvcx

RUN cargo update-version
RUN cargo test --color=never --no-default-features --features "ci sovtoken" -- --test-threads=1
RUN cargo update-so
RUN cargo deb --no-build
RUN find -type f -name "libvcx*.deb" -exec dpkg -i {} \;

CMD ["sh", "-c", "cp `find /sdk/vcx/libvcx/target/debian -type f -name \"*.deb\"` /sdk/vcx/output"]


