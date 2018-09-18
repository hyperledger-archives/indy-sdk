#!/bin/bash

# if [ $# -ne 2 ]; then
#     echo "USAGE: $0 CREDENTIALS FILE URL"
#     exit 1
# fi

CREDENTIALS=$KRAKEN_CREDENTIALS
FILENAME=$1
URL=$2
LOOKUP_DIR=$3

if [ -z "$LOOKUP_DIR" ]; then
    LOOKUP_DIR="./output"
fi


echo "Filename: ${FILENAME}"
echo "TYPE: ${TYPE}"
echo "URL: $URL"
echo "LOOKUP_DIR: $LOOKUP_DIR"

echo 'info:'
pwd
ls -al
echo 'end info'

find $LOOKUP_DIR -type f -name ${FILENAME} -exec curl -u $CREDENTIALS -X POST $URL -F 'file=@{}' \;

