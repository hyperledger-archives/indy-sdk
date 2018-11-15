#!/bin/bash
cd vcx/wrappers/node/
npm i
npm run lint
npm run compile
npm test
npm run test-logging