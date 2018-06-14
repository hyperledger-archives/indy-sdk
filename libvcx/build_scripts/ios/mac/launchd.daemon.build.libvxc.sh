#!/bin/bash

START_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $START_DIR
cd ../../../../..
git pull > ./vcx/libvcx/build_scripts/ios/mac/git.pull.out 2>&1
# git checkout libindy_refactor
# git pull
# git checkout mobile_vcx
# git merge libindy_refactor
cd vcx/libvcx/build_scripts/ios/mac
./mac.03.libindy.build.sh > ./mac.03.libindy.build.sh.out 2>&1
./mac.04.libvcx.setup.sh > ./mac.04.libvcx.setup.sh.out 2>&1
./mac.06.libvcx.build.sh > ./mac.06.libvcx.build.sh.out 2>&1
./mac.11.copy.static.libs.to.app.sh > ./mac.11.copy.static.libs.to.app.sh.out 2>&1
./mac.12.combine.static.libs.sh libvcxall delete > ./mac.12.combine.static.libs.sh.out 2>&1
