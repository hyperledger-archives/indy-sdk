# Running the Alice/Faber Python demo

There are 2 versions of this demo, the original version (faber.py and alice.py), and a slightly modified version (faber-pg.py and alice-pg.py) that supports postgres wallet storage, and illustrated how the vcx object serializtion/deserialization can be used.

## Original Demo

This demo consists of 3 files:

faber.py - a script that acts as an institution/enterprise by sending a connection request, writing a schema/cred_def to the ledger, sending a credential offer, and requesting proof.  

alice.py - a script that acts as an individual by accepting a connection offer, requesting a credential and offering proof.

pool.txn - genesis files for connecting to an indy pool (existing file connects to libindy/sovtoken ledger)

To run these follow the next steps:
 1) install the python requirements: `pip install -r requirements.txt`
 2) install a payment plugin -- [libnullpay](../../../../libnullpay/README.md#binaries)
 3) start Dummy Cloud Agent according to [instruction](../../../dummy-cloud-agent/README.md)
 4) execute the faber.py script first with `python3.5 faber.py`.
    This script will explain what it is doing and output invite details.
 5) When the invite details are displayed start the alice.py script with `python3.5 alice.py`.
    This script will ask for invite details which can copy/pasted from the output of the faber.py script.
 6) Once the connection is established the faber.py script will send a credential offer, credential and proof request automatically.
    The alice.py script will request a credential, store it and offer proof when asked.
    Once they have interacted they will both exit.

## Slightly Modified Demo

This demo is run using the following files:

faber-pg.py - same as faber.py, however once a connection is established, this script provides a menu:
                 1 = send credential to Alice
                 2 = send proof request to Alice
                 3 = poll the Alice connection to see if Alice has sent any messages
                 x = stop and exit

alice-pg.py - same as alice.py, however once a connection is established, this script provides a menu:
                 y = poll the connection for messages (credential offers or proof requests)
                 n = stop and exit

To create the alice/faber wallets using postgres storage, just add the "--postgres" option when running the script.

Internally, the scripts serialize and deserialize the vcx objects between operations.  In "real life", these serialized objects could be stored to the database or to wallet non-secrets storage.
