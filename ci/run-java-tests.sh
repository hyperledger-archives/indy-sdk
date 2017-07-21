#!/bin/bash

# Use this script because we can not change directory after docker.inside { }
# https://issues.jenkins-ci.org/browse/JENKINS-35518

cd wrappers/java

maven clean test
