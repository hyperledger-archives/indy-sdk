"""
Example demonstrating how to write Schema and Cred Definition on the ledger

As a setup, Steward (already on the ledger) adds Trust Anchor to the ledger.

After that, Steward builds the SCHEMA request to add new schema to the ledger.
Once that succeeds, Trust Anchor uses anonymous credentials to issue and store
claim definition for the Schema added by Steward.
"""


import asyncio
import json
import pprint

from indy import pool, ledger, wallet, signus, anoncreds
from indy.error import IndyError


pool_name = 'pool'
wallet_name = 'wallet'
genesis_file_path = '/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis'


def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))


async def write_schema_and_cred_def():
    try:
        # 1.
        print_log('\n1. Creates a new local pool ledger configuration that is used '
                  'later when connecting to ledger.\n')
        pool_config = json.dumps({'genesis_txn': genesis_file_path})
        await pool.create_pool_ledger_config(config_name=pool_name, config=pool_config)

        # 2.
        print_log('\n2. Open pool ledger and get handle from libindy\n')
        pool_handle = await pool.open_pool_ledger(config_name=pool_name, config=None)

        # 3.
        print_log('\n3. Creating new secure wallet\n')
        await wallet.create_wallet(pool_name, wallet_name, None, None, None)

        # 4.
        print_log('\n4. Open wallet and get handle from libindy\n')
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
        print_log('Trust anchor DID: ', trust_anchor_did)
        print_log('Trust anchor Verkey: ', trust_anchor_verkey)

        # 7.
        print_log('\n7. Building NYM request to add Trust Anchor to the ledger\n')
        nym_transaction_request = await ledger.build_nym_request(submitter_did=steward_did,
                                                                 target_did=trust_anchor_did,
                                                                 ver_key=trust_anchor_verkey,
                                                                 alias=None,
                                                                 role='TRUST_ANCHOR')
        print_log('NYM transaction request: ')
        pprint.pprint(json.loads(nym_transaction_request))

        # 8.
        print_log('\n8. Sending NYM request to the ledger\n')
        nym_transaction_response = await ledger.sign_and_submit_request(pool_handle=pool_handle,
                                                                        wallet_handle=wallet_handle,
                                                                        submitter_did=steward_did,
                                                                        request_json=nym_transaction_request)
        print_log('NYM transaction response: ')
        pprint.pprint(json.loads(nym_transaction_response))

        # 9.
        print_log('\n9. Build the SCHEMA request to add new schema to the ledger as a Steward\n')
        seq_no = 1
        schema = {
            'seqNo': seq_no,
            'dest': steward_did,
            'data': {
                'name': 'gvt',
                'version': '1.0',
                'attr_names': ['age', 'sex', 'height', 'name']
            }
        }
        schema_data = schema['data']
        print_log('Schema data: ')
        pprint.pprint(schema_data)
        print_log('Schema: ')
        pprint.pprint(schema)
        schema_request = await ledger.build_schema_request(steward_did, json.dumps(schema_data))
        print_log('Schema request: ')
        pprint.pprint(json.loads(schema_request))

        # 10.
        print_log('\n10. Sending the SCHEMA request to the ledger\n')
        schema_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, steward_did, schema_request)
        print_log('Schema response:')
        pprint.pprint(json.loads(schema_response))

        # 11.
        print_log('\n11. Creating and storing CRED DEFINITION using anoncreds as Trust Anchor, for the given Schema\n')
        claim_def_json = await anoncreds.issuer_create_and_store_claim_def(wallet_handle, trust_anchor_did, json.dumps(schema), 'CL', False)
        print_log('Claim Definition: ')
        pprint.pprint(json.loads(claim_def_json))

        # 12.
        print_log('\n12. Closing wallet and pool\n')
        await wallet.close_wallet(wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        # 13.
        print_log('\n13. Deleting created wallet\n')
        await wallet.delete_wallet(wallet_name, None)

        # 14.
        print_log('\n14. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)

    except IndyError as e:
        print('Error occurred: %s' % e)


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(write_schema_and_cred_def())
    loop.close()


if __name__ == '__main__':
    main()

