#!/bin/bash

if [ $# -ne 3 ]; then
    echo "USAGE: $0 CREDENTIALS FILE URL"
    exit 1
fi

CREDENTIALS=$1
FILENAME=$2
URL=$3
LOOKUP_DIR="/sdk/vcx/output"

echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"

find $LOOKUP_DIR -type f -name ${FILENAME} -exec curl -u $CREDENTIALS -X POST $URL -F 'file=@{}' \;

