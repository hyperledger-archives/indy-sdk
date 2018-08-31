#!/bin/bash

START_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $START_DIR
cd ../../../../..
git pull > ./vcx/libvcx/build_scripts/android/mac/git.pull.out 2>&1
# git checkout libindy_refactor
# git pull
# git checkout mobile_vcx
# git merge libindy_refactor
cd vcx/libvcx/build_scripts/android/mac
./mac.03.libindy.build.sh > ./mac.03.libindy.build.sh.out 2>&1
./mac.06.libvcx.build.sh > ./mac.06.libvcx.build.sh.out 2>&1
./mac.08.copy.shared.libs.to.app.sh > ./mac.08.copy.shared.libs.to.app.sh.out 2>&1
./mac.09.combine.shared.libs.sh > ./mac.09.combine.shared.libs.sh.out 2>&1
./mac.upload.android.build.zipfiles.sh > ./mac.upload.android.build.zipfiles.sh.out 2>&1
