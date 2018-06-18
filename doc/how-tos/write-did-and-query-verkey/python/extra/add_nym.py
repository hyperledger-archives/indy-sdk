'''
Created on January 30, 2018
@author: khoi.ngo

For updating role of an user in the ledger, a person or an organization in that
ledger must have one of these roles: Trustee, Steward or Trust Anchor.
(https://docs.google.com/spreadsheets/d/1TWXF7NtBjSOaUIBeIH77SyZnawfo91cJ_ns4TR-wsq4/edit#gid=0)

This script will setup an environment with a pool (from genesis_txn file),
a wallet inside that pool and get the default Steward DID
(what was added to the ledger during setup). Using that default Steward
to create a Trust Anchor.
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
  Step 5. Create then store DID and Verkey of seed_default_steward
          and seed_trust_anchor.
          The pair of keys of seed_default_steward will be used to make
          a request on step 6.

* Build and submit nym request to the ledger.
  Step 6. Build a nym request. Using default Steward create Trust Anchor
  Step 7. Submit the nym request and get the response.
          The new Trust Anchor is created.
'''

import asyncio
import json

from indy import wallet, did, pool, ledger
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
    seed_trustanchor = "TestTrustAnchor00000000000000000"
    wallet_credentials = json.dumps({"key": "wallet_key"})


def print_log(value_color):
    """set the colors for text."""
    OKGREEN = '\033[92m'
    ENDC = '\033[0m'
    print(OKGREEN + "\n" + value_color + ENDC)


async def build_nym_request():
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
                                   None, None, Variables.wallet_credentials)
        print_log("DONE")

        # 4. Get wallet handle to use in methods that require wallet access.
        print_log("4. Get wallet handle to use in methods that require "
                  "wallet access.")
        Variables.wallet_handle = await wallet.open_wallet(
            Variables.wallet_name,
            None, Variables.wallet_credentials)
        print_log("DONE - Wallet_handle: " + str(Variables.wallet_handle))

        # 5. Create then store DID and verkey of seed_default_steward,
        # seed_trustanchor. Steward DID with a verkey were already added
        # in the ledger with STEWARD role.
        print_log("5. Create then store steward DID and verkey "
                  "(for verification of signature)")
        seed_default_steward = "000000000000000000000000Steward1"

        (default_steward_did, _) = await did.create_and_store_my_did(
                    Variables.wallet_handle, json.dumps(
                        {"seed": seed_default_steward}))
        (trust_anchor_did, trust_anchor_verkey) = \
            await did.create_and_store_my_did(
                Variables.wallet_handle, json.dumps({}))
        print_log("DONE - Trust_Anchor[%s][%s]" % (trust_anchor_did,
                                                   trust_anchor_verkey))

        # 6. Prepare data and build the nym request.
        print_log("6. Build a nym request. Using default Steward "
                  "create Trust Anchor")
        nym_txn_req = await ledger.build_nym_request(
                                            default_steward_did,
                                            trust_anchor_did,
                                            trust_anchor_verkey, None,
                                            "TRUST_ANCHOR")
        print_log("DONE - nym_txn_req: " + str(nym_txn_req))

        # 7. Send the nym request and get the response.
        print_log("7. Send the nym request to the pool ledger")
        result = await ledger.sign_and_submit_request(
                                            Variables.pool_handle,
                                            Variables.wallet_handle,
                                            default_steward_did, nym_txn_req)
        print_log("DONE - Result: " + str(result))
    except IndyError as E:
        print(str(E))


# Create the loop instance using asyncio
loop = asyncio.get_event_loop()
loop.run_until_complete(build_nym_request())

# Close the loop instance
loop.close()
