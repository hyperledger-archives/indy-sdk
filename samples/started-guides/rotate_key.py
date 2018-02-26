'''
Created on January 30, 2018
@author: khoi.ngo

All SDK users can change their own verkey.
(https://docs.google.com/spreadsheets/d/1TWXF7NtBjSOaUIBeIH77SyZnawfo91cJ_ns4TR-wsq4/edit#gid=0)

This script will setup an environment with a pool (from genesis_txn file),
a wallet inside that pool and get the default Steward DID
(what was added to the ledger during setup). Using that default Steward
to create a Trust Anchor. Then rotating key for the new trust anchor.
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

* Build and submit nym request to the ledger.
  Step 6. Prepare data and build the nym request.
  Step 7. Send the nym request and get the response.

* Rotate key
  Step 8. Generates new keys for trust_anchor did and saves it as a new verkey.
  Step 9. Build a NYM request to update verkey on ledger.
  Step 10. Sign and submit built NYM request to ledger.
  Step 11. Apply new verkey as main for trust_anchor did.
  Step 12. Get verkey in wallet to make sure it was changed.
  Step 13. Build get NYM request to get verkey from ledger.
  Step 14. Submit built get NYM request and take verkey from response.
  Step 15. Verify that verkey in wallet is equal with verkey in ledger.
'''

import asyncio
import json

from indy import wallet, signus, pool, ledger


class Variables:
    # Initialize pool_handle, wallet_handle variable.
    # Set pool_name, wallet_name, seed and the pool configuration path.
    # Set a new seed_trustee with exactly 32 characters.
    pool_handle = 0
    wallet_handle = 0
    pool_name = "Evenym_pool1"
    wallet_name = "Evernym_wallet1"
    pool_txn = "/var/lib/indy/sandbox/pool_transactions_sandbox_genesis"


def print_log(value_color):
    """set the colors for text."""
    OKGREEN = '\033[92m'
    ENDC = '\033[0m'
    print(OKGREEN + "\n" + value_color + ENDC)


def print_error(message):
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    print(FAIL + "\n" + message + ENDC)


async def rotate_key():
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

        # 5. Create then store DID and verkey of seed_default_steward,
        # the trust anchor.Steward DID with a verkey were already added
        # in the ledger with STEWARD role.
        print_log("5. Create then store steward DID and verkey "
                  "(for verification of signature)")
        seed_default_steward = "000000000000000000000000Trustee1"

        (default_steward_did, _) = \
            await signus.create_and_store_my_did(
                Variables.wallet_handle, json.dumps(
                    {"seed": seed_default_steward}))
        print_log("DONE - Steward[%s]" % (default_steward_did))
        (did, verkey) = \
            await signus.create_and_store_my_did(
                Variables.wallet_handle, json.dumps({}))
        print_log("DONE - Trust_Anchor[%s][%s]" % (did,
                                                   verkey))

        # 6. Prepare data and build the nym request.
        print_log("6. Build a nym request. Using default Steward "
                  "create Trust Anchor")
        nym_txn_req = await ledger.build_nym_request(
            default_steward_did,
            did, verkey, None, "TRUST_ANCHOR")
        print_log("DONE - nym_txn_req: " + str(nym_txn_req))

        # 7. Send the nym request and get the response.
        print_log("7. Send the nym request to the pool ledger")
        result = await ledger.sign_and_submit_request(
            Variables.pool_handle,
            Variables.wallet_handle,
            default_steward_did, nym_txn_req)
        print_log("DONE - Result: " + str(result))

        # 8. Generates new keys for trust_anchor did and saves it
        # as a new verkey.
        print_log("8. Change verkey of trust anchor")
        new_verkey = await signus.replace_keys_start(
            Variables.wallet_handle,
            did, json.dumps({}))
        print_log("DONE - new verkey of trust anchor [%s]" % (new_verkey))

        # 9. Build NYM request to update new verkey to ledger.
        print_log("11. Build NYM request to update new verkey to ledger")
        nym_req = await ledger.build_nym_request(did, did, new_verkey, None,
                                                 'TRUST_ANCHOR')
        print_log('NYM request:\n{}'.format(nym_req))

        # 10. Sign and submit nym request to ledger.
        print_log("12. Sign and submit nym request to ledger")
        nym_response = await ledger.sign_and_submit_request(
            Variables.pool_handle, Variables.wallet_handle, did, nym_req)

        print_log('NYM response:\n{}'.format(nym_response))

        # 11. Apply new verkey as main for trust_anchor did.
        print_log("9. Apply new verkey")
        await signus.replace_keys_apply(Variables.wallet_handle, did)
        print_log("DONE")

        # 12. Get verkey in wallet to make sure it was changed.
        print_log("10. Get verkey in wallet")
        verkey_in_wallet = await signus.key_for_local_did(
            Variables.wallet_handle, did)
        if verkey_in_wallet != verkey and verkey_in_wallet == new_verkey:
            print_log("DONE - verkey in wallet was changed: " +
                      str(verkey_in_wallet))
        else:
            err = 'FAIL - verkey in wallet was not changed: {}'.format(
                verkey_in_wallet)
            print_error(err)
            raise ValueError(err)

        # 13. Build get NYM request to get verkey from ledger.
        print_log('13. Build get NYM request to get verkey from ledger')
        get_nym_req = await ledger.build_get_nym_request(did, did)

        print_log('DONE - created get NYM request:\n{}'.format(get_nym_req))

        # 14. Submit built get NYM request and take verkey from response.
        print_log('14. Submit built get NYM request '
                  'and take verkey from response')
        gotten_nym = await ledger.submit_request(Variables.pool_handle,
                                                 get_nym_req)

        print_log('DONE - gotten nym:\n{}'.format(gotten_nym))

        verkey_in_ledger = json.loads(json.loads(gotten_nym)['result']
                                      ['data'])['verkey']

        # 15. Verify that verkey in wallet is equal with verkey in ledger.
        if verkey_in_ledger != verkey and verkey_in_ledger == verkey_in_wallet:
            print_log("DONE - verkey in ledger was changed: " +
                      str(verkey_in_ledger))
        else:
            err = 'FAIL - verkey in ledger was not changed: {}'.format(
                verkey_in_ledger)
            print_error(err)
            raise ValueError(err)

    except Exception as e:
        print_error(str(e))


# Create the loop instance using asyncio
loop = asyncio.get_event_loop()
loop.run_until_complete(rotate_key())

# Close the loop instance
loop.close()
