cd ../../../cli && \
RUST_BACKTRACE=1 LD_LIBRARY_PATH=../experimental/plugins/postgres_storage/target/debug/ cargo run ../experimental/plugins/postgres_storage/cli_ps_test.txt
