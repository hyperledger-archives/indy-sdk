#!/bin/bash

npm -v
npm run lint
npm run compile
npm test
npm pack
test -f vcx-*.tgz
