#!/bin/bash
export VCX_VERSION=$(python3 /sdk/vcx/ci/scripts/toml_utils.py /sdk/vcx/libvcx/Cargo.toml)
cd /sdk/vcx/wrappers/python3/tests
pytest -s
cd /sdk/vcx/wrappers/python3/
python3 setup.py sdist
cp /sdk/vcx/wrappers/python3/dist/*.tar.gz /sdk/vcx/output

