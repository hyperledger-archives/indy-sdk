#!/bin/bash
mkdir -p /tmp/cert
if [ ! -f /tmp/cert/ca.crt ] ; then     
    curl -k -o /tmp/cert/ca.crt https://repo.corp.evernym.com/ca.crt      
fi