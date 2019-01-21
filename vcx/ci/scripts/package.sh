#!/bin/bash
set -e
OUTPUTDIR=output
CURDIR=$(pwd)
export PATH=${PATH}:$(pwd)/vcx/ci/scripts
cd vcx/libvcx/
echo "Updating Version in Cargo.toml file"
cargo update-version
echo "Updating Cargo"
if [ "$1" != "--no-test" ]; then
    echo "Testing libvcx.so"
    cargo test --no-default-features --features "ci" -- --test-threads=1
fi
echo "Building libvcx.so"
cargo build --no-default-features --features "ci"
echo "Updating libvcx.so File with Version"
cargo update-so
echo "Creating Libvcx Debian File"
cargo deb --no-build
echo "Moving Libvcx Debian File to Output Directory"
cp target/debian/*.deb $CURDIR/$OUTPUTDIR


