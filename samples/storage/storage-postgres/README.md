# Sample storage plugin in a Docker container

Note everything is pegged to release v1.8.1
```
git checkout tags/v1.8.1
```

Start docker containers
```
docker-compose build
docker-compose up
docker exec -it postgres-wallet bash
```

Build projects (note: we could also do these steps in the Dockerfile but this allows changing things)
```
PATH=$PATH:~/.cargo/bin
cargo build --manifest-path libindy/Cargo.toml --release
cargo build --manifest-path cli/Cargo.toml --release
cargo build --manifest-path experimental/plugins/postgres_storage/Cargo.toml --release
```

Run the cargo tests:
```
cd experimental/plugins/postgres_storage
RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1
```

Run the cli test:
```
cd cli
RUST_BACKTRACE=1 LD_LIBRARY_PATH=/indy-sdk/experimental/plugins/postgres_storage/target/release/ cargo run /indy-sdk/samples/storage/storage-postgres/cli_ps_test.txt
```

Also including one sample test for nodejs:
```
cd samples/storage/storage-postgres
npm install
node src/testStoragePlugin.js
```

If you need to copy out the built libindystrgpostgres.so file from the docker container:
```
docker cp postgres-wallet:/indy-sdk/experimental/plugins/postgres_storage/target/release/libindystrgpostgres.so /tmp/libindystrgpostgres.so
```
