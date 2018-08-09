#!/bin/bash

./mac.03.x86_64.libindy.build.sh
./mac.06.x86_64.libvcx.build.sh
./mac.08.x86_64.copy.shared.libs.to.app.sh
./mac.09.x86_64.combine.shared.libs.sh
