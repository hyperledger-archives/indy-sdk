How to package debians for the libvcx library and associated wrappers (as a dev):

01) Add ./sdk/vcx/ci/scripts to your PATH variable

02) Build Release binary of libvcx.so
    '$ cargo build'

03) Move to ./sdk/vcx/libvcx directory.

04) Update Cargo.toml and package.json files to current version/build.
    '$ cargo update-version'

05) Update libvcx/release/libvcx.so to newest version.
    '$ cargo update-so'

06) Create debian package
    '$ cargo deb --no-build'

07) Gzip so file
    '$ python ./sdk/vcx/ci/scripts/gzip_so_file.py ./sdk/vcx/libvcx/target/debug/libvcx.so.<version> DEST_DIR'

08) Copy target/debug/libvcx.so to ./sdk/vcx/wrapers/node/lib

09) Change directories to './sdk/vcx/wrappers/node'

10) Package npm module
    '$ npm pack'

11) Run ./sdk/vcx/ci/scripts/create_npm_deb.py
    '$ create_npm_deb.py node-vcx-<version>.tgz'

