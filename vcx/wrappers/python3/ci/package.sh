#!/bin/bash
set -e
export PATH=${PATH}:$(pwd)/vcx/ci/scripts
export VCX_VERSION=$(toml_utils.py vcx/libvcx/Cargo.toml)
export DIST=`pwd`/vcx/wrappers/python3/dist/
export PACKAGE_NAME='python3-vcx-wrapper'
pushd vcx/wrappers/python3
python3 setup.py sdist
PACKAGE=${PACKAGE_NAME}-${VCX_VERSION}.tar.gz
FILELY_PACKAGE=${PACKAGE_NAME}_${VCX_VERSION}.tar.gz
# Added test so that we can confirm the new package name and that it was created.
if [ ! -e ${DIST}/${PACKAGE} ]; then
    echo "Python Package Not Created"
    exit 1
fi
popd
mv ${DIST}/${PACKAGE} output/${FILELY_PACKAGE}
