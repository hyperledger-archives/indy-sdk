#!/bin/bash
OUTPUTDIR=/sdk/vcx/output
npm ci
npm run lint
npm run compile
npm test
npm pack

rename \s/vcx-/vcx_/ *.tgz
rename \s/\\.tgz\$/_amd64\\.tgz/ *.tgz

cp *.tgz $OUTPUTDIR

find $OUTPUTDIR -type f -name 'vcx_*.tgz' -exec python3 /sdk/vcx/ci/scripts/create_npm_deb.py {} \;
find -type f -name 'vcx_*.deb' -exec cp {} $OUTPUTDIR \;


