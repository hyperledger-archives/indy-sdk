#!/usr/bin/env bash

CUSTOM_DOCKER_TAG="$1"

if [ -z "$CUSTOM_DOCKER_TAG" ]; then
  VCX_TOML_PATH="`dirname $0`/../libvcx/Cargo.toml"
  VCX_VERSION=`cat $VCX_TOML_PATH | grep "^version = \".*\"" | grep -o "[0-9]*\.[0-9]*\.[0-9]*"`
  DOCKER_TAG="vcx-agency:${VCX_VERSION}"
  echo "Note: You can run this script with argument to use custom tag, eg: ./dockerbuild.sh vcx-agency:1.2.3"
else
  DOCKER_TAG="$CUSTOM_DOCKER_TAG"
fi;

echo "This will build ${DOCKER_TAG}"
echo "Do you want to continue? (y/n)"
read yesno
if [ $yesno != "y" ]; then
  exit 0
fi


mkdir tmpdocker
rsync -avr --exclude='target' --exclude='**/.idea' --exclude='rpm' --exclude='tests' --exclude='debian' ../../libindy tmpdocker
rsync -avr --exclude='target' --exclude='**/.idea' --exclude='rpm' --exclude='tests' --exclude='debian' --exclude='**/node_modules/' --exclude='**/build/' ../../wrappers tmpdocker

rsync -avr --exclude='target' --exclude='**/.idea' --exclude='rpm' --exclude='tests' --exclude='debian' \
     --exclude='**/node_modules/' --exclude='**/build/' --exclude='**/dist/' --exclude='**/doc/' --exclude='dummy-cloud-agent' --exclude='ci' --exclude='docs' \
     --exclude='samples' \
     ../../vcx tmpdocker
rsync -avr --exclude='target' --exclude='**/.idea' --exclude='rpm' --exclude='tests' --exclude='debian' ../../libnullpay tmpdocker



docker build -t "${DOCKER_TAG}" .
rm -r tmpdocker