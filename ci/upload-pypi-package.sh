#!/bin/bash -x

cd wrappers/python

python3.6 setup.py register -r pypi

python3.6 setup.py sdist upload -r pypi