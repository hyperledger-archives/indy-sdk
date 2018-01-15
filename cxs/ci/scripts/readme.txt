How to package debians for the libcxs library and associated wrappers (as a dev):

01) Add ./sdk/cxs/ci/scripts to your PATH variable

02) Build Release binary of libcxs.so
    '$ cargo build'

03) Move to ./sdk/cxs/libcxs directory.

04) Update Cargo.toml and package.json files to current version/build.
    '$ cargo update-version'

05) Update libcxs/release/libcxs.so to newest version.
    '$ cargo update-so'

06) Create debian package
    '$ cargo deb --no-build'

07) Gzip so file
    '$ python ./sdk/cxs/ci/scripts/gzip_so_file.py ./sdk/cxs/libcxs/target/debug/libcxs.so.<version> DEST_DIR'

08) Copy target/debug/libcxs.so to ./sdk/cxs/wrapers/node/lib

09) Change directories to './sdk/cxs/wrappers/node'

10) Package npm module
    '$ npm pack'

11) Run ./sdk/cxs/ci/scripts/create_npm_deb.py
    '$ create_npm_deb.py node-cxs-<version>.tgz'

