#!/bin/bash
export PATH=${PATH}:$(pwd)/vcx/ci/scripts
export VCX_VERSION=$(toml_utils.py vcx/libvcx/Cargo.toml)
pushd vcx/wrappers/python3
python3 setup.py sdist
popd
cp vcx/wrappers/python3/dist/*.tar.gz output
