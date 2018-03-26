"""
This sample is extensions of "write_schema_and_cred_def.py"

Shows how to issue a credential as a Trust Anchor which has created a Cred Definition
for an existing Schema.

After Trust Anchor has successfully created and stored a Cred Definiton using Anonymous Credentials,
Prover's wallet is created and opened, and used to generate Prover's Master Secret.
After that, Trust Anchor generates Claim Offer for given Cred Definition, using Prover's DID
Prover uses Claim Offer to create Claim Request
Trust Anchor then uses Prover's Claim Request to issue a Claim.
Finally, Prover stores Claim in its wallet.
"""


import asyncio
import json
import pprint
import sys
sys.path.insert(0, '/home/vagrant/code/evernym/indy-sdk/wrappers/python')


from indy import pool, ledger, wallet, did, anoncreds
from indy.error import IndyError


pool_name = 'pool'
wallet_name = 'wallet'
genesis_file_path = '/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis'


def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))


async def issue_credential():
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
        print_log('\n3. Creating new secure wallet_handle\n')
        await wallet.create_wallet(pool_name, wallet_name, None, None, None)

        # 4.
        print_log('\n4. Open wallet_handle and get handle from libindy\n')
        wallet_handle = await wallet.open_wallet(wallet_name, None, None)

        # 5.
        print_log('\n5. Generating and storing steward DID and verkey\n')
        steward_seed = '000000000000000000000000Steward1'
        did_json = json.dumps({'seed': steward_seed})
        steward_did, steward_verkey = await did.create_and_store_my_did(wallet_handle, did_json)
        print_log('Steward DID: ', steward_did)
        print_log('Steward Verkey: ', steward_verkey)

        # 6.
        print_log('\n6. Generating and storing trust anchor DID and verkey\n')
        trust_anchor_did, trust_anchor_verkey = await did.create_and_store_my_did(wallet_handle, "{}")
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
        print_log('Schema: ')
        pprint.pprint(schema_data)
        schema_request = await ledger.build_schema_request(steward_did, json.dumps(schema_data))
        print_log('Schema request: ')
        pprint.pprint(json.loads(schema_request))

        # 10.
        print_log('\n10. Sending the SCHEMA request to the ledger\n')
        schema_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, steward_did, schema_request)
        print_log('Schema response:')
        pprint.pprint(json.loads(schema_response))

        # 11.
        print_log('\n11. Creating and storing CLAIM DEFINITION using anoncreds as Trust Anchor, for the given schema')
        claim_def_json = await anoncreds.issuer_create_and_store_claim_def(wallet_handle, trust_anchor_did, json.dumps(schema), 'CL', False)
        print_log('Claim Definition: ')
        pprint.pprint(json.loads(claim_def_json))

        # 12.
        print_log('\n12. Creating Prover wallet and opening it to get the handle\n')
        prover_did = 'VsKV7grR1BUE29mG2Fm2kX'
        prover_wallet_name = 'prover_wallet'
        await wallet.create_wallet(pool_name, prover_wallet_name, None, None, None)
        prover_wallet_handle = await wallet.open_wallet(prover_wallet_name, None, None)

        # 13.
        print_log('\n13. Prover is creating Master Secret\n')
        master_secret_name = 'master_secret'
        await anoncreds.prover_create_master_secret(prover_wallet_handle, master_secret_name)

        # 14.
        print_log('\n14. Issuer (Trust Anchor) is creating a Claim Offer for Prover\n')
        schema_json = json.dumps(schema)
        claim_offer_json = await anoncreds.issuer_create_claim_offer(wallet_handle, schema_json, trust_anchor_did, prover_did)
        print_log('Claim Offer: ')
        pprint.pprint(json.loads(claim_offer_json))
        
        # 15.
        print_log('\n15. Prover creates Claim Request\n')
        claim_req_json = await anoncreds.prover_create_and_store_claim_req(prover_wallet_handle, prover_did, claim_offer_json, claim_def_json, master_secret_name)
        print_log('Claim Request: ')
        pprint.pprint(json.loads(claim_req_json))
        
        # 16.
        print_log('\n16. Issuer (Trust Anchor) creates Claim for Claim Request\n')
        claim_json = json.dumps({
            'sex': ['male', '5944657099558967239210949258394887428692050081607692519917050011144233115103'],
            'name': ['Alex', '1139481716457488690172217916278103335'],
            'height': ['175', '175'],
            'age': ['28', '28']
        })
        _, claim_json = await anoncreds.issuer_create_claim(wallet_handle, claim_req_json, claim_json, -1)
        print_log('Claim: ')
        pprint.pprint(json.loads(claim_json))

        # 17.
        print_log('\n17. Prover processes and stores Claim\n')
        await anoncreds.prover_store_claim(prover_wallet_handle, claim_json, None)

        # 18.
        print_log('\n18. Closing both wallet_handles and pool\n')
        await wallet.close_wallet(wallet_handle)
        await wallet.close_wallet(prover_wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        # 19.
        print_log('\n19. Deleting created wallet_handles\n')
        await wallet.delete_wallet(wallet_name, None)
        await wallet.delete_wallet(prover_wallet_name, None)

        # 20.
        print_log('\n20. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)

    except IndyError as e:
        print('Error occurred: %s' % e)


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(issue_credential())
    loop.close()


if __name__ == '__main__':
    main()

