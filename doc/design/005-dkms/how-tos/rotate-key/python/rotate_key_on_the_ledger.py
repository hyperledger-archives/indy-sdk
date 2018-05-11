"""
Example demonstrating how to do the key rotation on the ledger.

Steward already exists on the ledger and its DID/Verkey are obtained using seed.
Trust Anchor's DID/Verkey pair is generated and stored into wallet.
Stewards builds NYM request in order to add Trust Anchor to the ledger.
Once NYM transaction is done, Trust Anchor wants to change its Verkey.
First, temporary key is created in the wallet.
Second, Trust Anchor builds NYM request to replace the Verkey on the ledger.
Third, when NYM transaction succeeds, Trust Anchor makes new Verkey permanent in wallet
(it was only temporary before).

To assert the changes, Trust Anchor reads both the Verkey from the wallet and the Verkey from the ledger
using GET_NYM request, to make sure they are equal to the new Verkey, not the original one
added by Steward
"""


import asyncio
import json
import pprint

from indy import pool, ledger, wallet, signus
from indy.error import IndyError


pool_name = 'pool'
wallet_name = 'wallet'
genesis_file_path = '/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis'


def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))


async def rotate_key_on_the_ledger():
    try:
        # 1.
        print_log('1. Creates a new local pool ledger configuration that is used '
                  'later when connecting to ledger.\n')
        pool_config = json.dumps({'genesis_txn': genesis_file_path})
        await pool.create_pool_ledger_config(config_name=pool_name, config=pool_config)

        # 2.
        print_log('\n2. Open pool ledger and get handle from libindy\n')
        pool_handle = await pool.open_pool_ledger(config_name=pool_name, config=None)

        # 3.
        print_log('\n3. Creating new secure wallet with the given unique name\n')
        await wallet.create_wallet(pool_name, wallet_name, None, None, None)

        # 4.
        print_log('\n4. Open wallet and get handle from libindy to use in methods that require wallet access\n')
        wallet_handle = await wallet.open_wallet(wallet_name, None, None)

        # 5.
        print_log('\n5. Generating and storing steward DID and verkey\n')
        steward_seed = '000000000000000000000000Steward1'
        did_json = json.dumps({'seed': steward_seed})
        steward_did, steward_verkey = await signus.create_and_store_my_did(wallet_handle, did_json)
        print_log('Steward DID: ', steward_did)
        print_log('Steward Verkey: ', steward_verkey)

        # 6.
        print_log('\n6. Generating and storing trust anchor DID and verkey\n')
        trust_anchor_did, trust_anchor_verkey = await signus.create_and_store_my_did(wallet_handle, "{}")
        print_log('Trust Anchor DID: ', trust_anchor_did)
        print_log('Trust Anchor Verkey: ', trust_anchor_verkey)

        # 7.
        print_log('\n7. Building NYM request to add Trust Anchor to the ledger\n')
        nym_transaction_request = await ledger.build_nym_request(submitter_did=steward_did,
                                                                 target_did=trust_anchor_did,
                                                                 ver_key=trust_anchor_verkey,
                                                                 alias=None,
                                                                 role='TRUST_ANCHOR')
        print_log('NYM request: ')
        pprint.pprint(json.loads(nym_transaction_request))

        # 8.
        print_log('\n8. Sending NYM request to the ledger\n')
        nym_transaction_response = await ledger.sign_and_submit_request(pool_handle=pool_handle,
                                                                        wallet_handle=wallet_handle,
                                                                        submitter_did=steward_did,
                                                                        request_json=nym_transaction_request)
        print_log('NYM response: ')
        pprint.pprint(json.loads(nym_transaction_response))

        # 9.
        print_log('\n9. Generating new verkey of trust anchor in wallet\n')
        new_verkey = await signus.replace_keys_start(wallet_handle, trust_anchor_did, "{}")
        print_log('New Trust Anchor Verkey: ', new_verkey)

        # 10.
        print_log('\n10. Building NYM request to update new verkey to ledger\n')
        nym_request = await ledger.build_nym_request(trust_anchor_did, trust_anchor_did, new_verkey, None, 'TRUST_ANCHOR')
        print_log('NYM request:')
        pprint.pprint(json.loads(nym_request))

        # 11.
        print_log('\n11. Sending NYM request to the ledger\n')
        nym_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, trust_anchor_did, nym_request)
        print_log('NYM response:')
        pprint.pprint(json.loads(nym_response))

        # 12.
        print_log('\n12. Apply new verkey in wallet\n')
        await signus.replace_keys_apply(wallet_handle, trust_anchor_did)

        # 13.
        print_log('\n13. Reading new verkey from wallet\n')
        verkey_in_wallet = await signus.key_for_local_did(wallet_handle, trust_anchor_did)
        print_log('Trust Anchor Verkey in wallet: ', verkey_in_wallet)

        # 14.
        print_log('\n14. Building GET_NYM request to get Trust Anchor verkey\n')
        get_nym_request = await ledger.build_get_nym_request(trust_anchor_did, trust_anchor_did)
        print_log('Get NYM request:')
        pprint.pprint(json.loads(get_nym_request))

        # 15.
        print_log('\n15. Sending GET_NYM request to ledger\n')
        get_nym_response_json = await ledger.submit_request(pool_handle, get_nym_request)
        get_nym_response = json.loads(get_nym_response_json)
        print_log('GET NYM response:')
        pprint.pprint(get_nym_response)

        # 16.
        print_log('\n16. Comparing Trust Anchor verkeys: written by Steward (original), '
                  'current in wallet and current from ledger\n')
        print_log('Written by Steward: ', trust_anchor_verkey)
        print_log('Current in wallet: ', verkey_in_wallet)
        verkey_from_ledger = json.loads(get_nym_response['result']['data'])['verkey']
        print_log('Current from ledger: ', verkey_from_ledger)
        print_log('Matching: ', verkey_from_ledger == verkey_in_wallet != trust_anchor_verkey)

        # 17.
        print_log('\n17. Closing wallet and pool\n')
        await wallet.close_wallet(wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        # 18.
        print_log('\n18. Deleting created wallet\n')
        await wallet.delete_wallet(wallet_name, None)

        # 19.
        print_log('\n19. Deleting pool ledger config')
        await pool.delete_pool_ledger_config(pool_name)

    except IndyError as e:
        print('Error occurred: %s' % e)


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(rotate_key_on_the_ledger())
    loop.close()


if __name__ == '__main__':
    main()

