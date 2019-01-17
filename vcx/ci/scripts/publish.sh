#!/bin/bash
set +x
set -e
if [ $# -ne 3 ]; then
    echo "USAGE: $0 CREDENTIALS FILE URL"
    exit 1
fi

CREDENTIALS=$1
FILENAME=$2
URL=$3
LOOKUP_DIR="output"

echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"

echo 'info:'
pwd
ls -al
echo 'end info'

find "./output" -type f -name ${FILENAME} -exec curl -u $CREDENTIALS -X POST $URL -F 'file=@{}' \;


