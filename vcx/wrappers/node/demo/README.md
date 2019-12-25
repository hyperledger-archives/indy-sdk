# Running the Alice/Faber NodeJS demo

This demo consists of 2 main files:

faber.js - a script that acts as an institution/enterprise by sending a connection request, writing a schema/cred_def to the ledger, sending a credential offer, and requesting proof.  

alice.js - a script that acts as an individual by accepting a connection offer, requesting a credential and offering proof.

docker.txn - genesis files for connecting to an indy pool (existing file connects to libindy/sovtoken ledger)

To run these follow the next steps:
 1) install the latest [vcx python package](../README.md#how-to-install)
 2) install a payment plugin -- [libnullpay](../../../../libnullpay/README.md#binaries)
 3) start Dummy Cloud Agent according to [instruction](../../../dummy-cloud-agent/README.md)
 4) execute the faber.py script first with `npm run demo:alice`.
    This script will explain what it is doing and output invite details.
 5) When the invite details are displayed start the alice.py script with `npm run demo:faber`.
    This script will ask for invite details which can copy/pasted from the output of the faber.py script.
 6) Once the connection is established the faber.py script will send a credential offer, credential and proof request automatically.
    The alice.py script will request a credential, store it and offer proof when asked.
    Once they have interacted they will both exit.
 
