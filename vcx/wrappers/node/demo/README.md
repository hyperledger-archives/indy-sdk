# Running the Alice/Faber NodeJS demo

This demo consists of 2 main files:

faber.js - a script that acts as an institution/enterprise by sending a connection request, writing a schema/cred_def to the ledger, sending a credential offer, and requesting proof.  

alice.js - a script that acts as an individual by accepting a connection offer, requesting a credential and offering proof.

docker.txn - genesis files for connecting to an indy pool (existing file connects to libindy/sovtoken ledger)

To run these follow the next steps:
 1) install the latest [vcx python package](../README.md#how-to-install)
 2) install a payment plugin -- [libnullpay](../../../../libnullpay/README.md#binaries)
 3) start Dummy Cloud Agent according to [instruction](../../../dummy-cloud-agent/README.md)
 
