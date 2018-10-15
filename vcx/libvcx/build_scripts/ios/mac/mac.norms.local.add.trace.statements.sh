
#!/bin/bash

source ./shared.functions.sh

START_DIR=$PWD
VCX_SDK=$START_DIR/../../../../..
VCX_SDK=$(abspath "$VCX_SDK")
WORK_DIR=$VCX_SDK/.macosbuild
WORK_DIR=$(abspath "$WORK_DIR")

python ${START_DIR}/../../add.trace.statements.to.src.py /Users/norm/forge/work/code/evernym/sdk-evernym/vcx/libvcx/src println
python ${START_DIR}/../../add.trace.statements.to.src.py /Users/norm/forge/work/code/evernym/sdk-evernym/.macosbuild/vcx-indy-sdk/libindy/src println
