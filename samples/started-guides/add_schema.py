'''
Created on January 30, 2018
@author: khoi.ngo

For writing a new schema in the ledger, a person or an organization in that
ledger must have one of these roles: Trustee, Steward or Trust Anchor.
(https://docs.google.com/spreadsheets/d/1TWXF7NtBjSOaUIBeIH77SyZnawfo91cJ_ns4TR-wsq4/edit#gid=0)

This script will setup an environment with a pool (from genesis_txn file),
a wallet inside that pool and get the default Steward DID
(what was added to the ledger during setup). Using that default Steward
to build a schema request. Then submit it to the ledger.
Refer to below for detail steps.

* Setup an environment:
  Step 1. Create the pool ledger from genesis txn file
          A genesis txn file contains the information about the NODE and
          CLIENT machines. With this information after dumped to JSON format
          and a name, a pool ledger is created.
  Step 2. Open pool ledger to get the pool handle via pool name.
          The returned pool handle is used in methods that require pool
          connection
  Step 3. Create the new Wallet.
  Step 4. Get wallet handle to use in methods that require wallet access.

* Create then store DID and Verkey of the seed_default_steward.
  This DID and Verkey was already added to the ledger and
  it is a default test Steward that happens during setup.
  We will use this Steward to send the schema request
  without creating a new one.
  Step 5. Create then store DID and Verkey of seed_default_steward.
          The pair of keys will be used to make a request on step 6.

* Build and submit schema request to the ledger.
  Step 6. Prepare data and build the schema_request.
  Step 7. Submit the schema request and get the response.
          The new schema is written to the ledger.
'''

import asyncio
import json

from indy import wallet, signus, pool, ledger
from indy.error import IndyError


class Variables:
    # Initialize pool_handle, wallet_handle variable.
    # Set pool_name, wallet_name, seed and the pool configuration path.
    # Set a new seed_trustee with exactly 32 characters.
    pool_handle = 0
    wallet_handle = 0
    pool_name = "Evenym_pool"
    wallet_name = "Evernym_wallet"
    pool_txn = "/var/lib/indy/sandbox/pool_transactions_sandbox_genesis"


def print_log(value_color):
    """set the colors for text."""
    OKGREEN = '\033[92m'
    ENDC = '\033[0m'
    print(OKGREEN + "\n" + value_color + ENDC)


async def build_schema_request():
    try:

        # 1. Create the pool ledger from genesis txn file
        # A genesis txn file contains the information about the NODE and
        # CLIENT machines. With this information after dumped to JSON format
        # and a name, a pool ledger is created.
        print_log("1. Creates a new local pool ledger configuration that "
                  "can be used later to connect pool nodes.")
        pool_config = json.dumps({"genesis_txn": str(Variables.pool_txn)})
        await pool.create_pool_ledger_config(Variables.pool_name, pool_config)
        print_log("DONE")
        # 2. Open pool ledger to get the pool handle via pool name.
        # The returned pool handle is used in methods that require pool
        # connection
        print_log("2. Open pool ledger and get the pool handle.")
        Variables.pool_handle = await pool.open_pool_ledger(
            Variables.pool_name, None)
        print_log("DONE - Pool handle: " + str(Variables.pool_handle))

        # 3. Create a new Wallet in the pool.
        print_log("3. Creates a new secure wallet with the given unique name.")
        await wallet.create_wallet(Variables.pool_name,
                                   Variables.wallet_name,
                                   None, None, None)
        print_log("DONE")

        # 4. Get wallet handle to use in methods that require wallet access.
        print_log("4. Get wallet handle to use in methods that require "
                  "wallet access.")
        Variables.wallet_handle = await wallet.open_wallet(
            Variables.wallet_name,
            None, None)
        print_log("DONE - Wallet_handle: " + str(Variables.wallet_handle))

        # 5. Create then store DID and verkey of seed_default_steward.
        # This DID and verkey were already added in the ledger
        # with STEWARD role.
        print_log("5. Create then store steward DID and verkey "
                  "(for verification of signature)")
        seed_default_steward = "000000000000000000000000Steward1"

        (default_steward_did, default_steward_verkey) = \
            await signus.create_and_store_my_did(
                    Variables.wallet_handle, json.dumps(
                        {"seed": seed_default_steward}))
        print_log("DONE - Steward [%s][%s]" % (default_steward_did,
                                               default_steward_verkey))

        # 6. Prepare data and build the schema_request.
        print_log("6. Build a schema request")
        name = "Transcript"
        version = "1.2"
        attributes = '["student_name", "ssn", "degree", "year", "status"]'
        data = ('{"name":"%s", "version":"%s", "attr_names":%s}' %
                (name, version, attributes))
        schema_req = await ledger.build_schema_request(default_steward_did,
                                                       data)
        print_log("DONE - schema_req: " + str(schema_req))

        # 7. send the schema request and get the result.
        print_log("7. Send the schema request to the pool ledger")
        result = await ledger.sign_and_submit_request(Variables.pool_handle,
                                                      Variables.wallet_handle,
                                                      default_steward_did,
                                                      schema_req)
        print_log("DONE - Result: " + str(result))
    except IndyError as E:
        print(str(E))


# Create the loop instance using asyncio
loop = asyncio.get_event_loop()
loop.run_until_complete(build_schema_request())

# Close the loop instance
loop.close()
