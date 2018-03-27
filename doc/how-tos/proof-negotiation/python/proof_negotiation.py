"""
Example demonstrating Proof Verification.

First Issuer creates Claim Definition for existing Schema.
After that, it issues a Claim to Prover (as in issue_credential.py example)

Once Prover has successfully stored its Claim, it uses Proof Request that he
received, to get Claims which satisfy the Proof Request from his wallet.
Prover uses the output to create Proof, using its Master Secret.
After that, Proof is verified against the Proof Request
"""

import asyncio
import json
import pprint
import sys

sys.path.insert(0, '/home/vagrant/code/evernym/indy-sdk/wrappers/python')

from indy import pool, ledger, wallet, did, anoncreds, crypto
from indy.error import IndyError


seq_no = 1
pool_name = 'pool'
issuer_wallet_name = 'issuer_wallet'
prover_wallet_name = 'prover_wallet'
issuer_did = 'NcYxiDXkpYi6ov5FcYDi1e'
prover_did = 'VsKV7grR1BUE29mG2Fm2kX'
genesis_file_path = '/home/vagrant/code/evernym/indy-sdk/cli/docker_pool_transactions_genesis'

def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))


async def proof_negotiation():
    try:
        # 1.
        print_log('\n1. Creates Issuer wallet and opens it to get handle.\n')
        await wallet.create_wallet(pool_name, issuer_wallet_name, None, None, None)
        issuer_wallet_handle = await wallet.open_wallet(issuer_wallet_name, None, None)

        # 2.
        print_log('\n2. Creates Prover wallet and opens it to get handle.\n')
        await wallet.create_wallet(pool_name, prover_wallet_name, None, None, None)
        prover_wallet_handle = await wallet.open_wallet(prover_wallet_name, None, None)

        # 3.
        print_log('\n3. Issuer creates Claim Definition for Schema\n')
        schema = {
            'seqNo': seq_no,
            'dest': issuer_did,
            'data': {
                'name': 'gvt',
                'version': '1.0',
                'attr_names': ['age', 'sex', 'height', 'name']
            }
        }
        schema_json = json.dumps(schema)
        schema_key = {
            'name': schema['data']['name'],
            'version': schema['data']['version'],
            'did': schema['dest'],
        }
        claim_def_json = await anoncreds.issuer_create_and_store_claim_def(issuer_wallet_handle, issuer_did, schema_json, 'CL', False)
        print_log('Claim Definition: ')
        pprint.pprint(json.loads(claim_def_json))

        # 4.
        print_log('\n4. Prover creates Master Secret\n')
        master_secret_name = 'master_secret'
        await anoncreds.prover_create_master_secret(prover_wallet_handle, master_secret_name)

        # 5.
        print_log('\n5. Issuer create Claim Offer\n')
        claim_offer_json = await anoncreds.issuer_create_claim_offer(issuer_wallet_handle, schema_json, issuer_did, prover_did)
        print_log('Claim Offer: ')
        pprint.pprint(json.loads(claim_offer_json))

        # 6.
        print_log('\n6. Prover creates and stores Claim request\n')
        claim_req_json = await anoncreds.prover_create_and_store_claim_req(prover_wallet_handle, prover_did, claim_offer_json,
                                                                           claim_def_json, master_secret_name)
        print_log('Claim Request: ')
        pprint.pprint(json.loads(claim_req_json))

        # 7.
        print_log('\n7. Issuer creates Claim for received Claim Request\n')
        claim_json = json.dumps({
            'sex': ['male', '5944657099558967239210949258394887428692050081607692519917050011144233115103'],
            'name': ['Alex', '1139481716457488690172217916278103335'],
            'height': ['175', '175'],
            'age': ['28', '28']
        })
        (_, claim_json) = await anoncreds.issuer_create_claim(issuer_wallet_handle, claim_req_json, claim_json, -1)

        # 8.
        print_log('\n8. Prover processes and stores received Claim\n')
        await anoncreds.prover_store_claim(prover_wallet_handle, claim_json, None)


        # 9.
        print_log('\n9. Prover gets Claims for Proof Request\n')
        proof_request = {
            'nonce': '123432421212',
            'name': 'proof_req_1',
            'version': '0.1',
            'requested_attrs': {
                'attr1_referent': {
                    'name': 'name',
                    'restrictions': [{
                        'issuer_did': issuer_did,
                        'schema_key': schema_key
                    }]
                }
            },
            'requested_predicates': {
                'predicate1_referent': {
                    'attr_name': 'age',
                    'p_type': '>=',
                    'value': 18,
                    'restrictions': [{'issuer_did': issuer_did}]
                }
            }
        }
        print_log('Proof Request: ')
        pprint.pprint(proof_request)
        proof_req_json = json.dumps(proof_request)
        claims_for_proof_request_json = await anoncreds.prover_get_claims_for_proof_req(prover_wallet_handle, proof_req_json)
        claims_for_proof_request = json.loads(claims_for_proof_request_json)
        print_log('Claims for Proof Request: ')
        pprint.pprint(claims_for_proof_request)

        # 10.
        print_log('\n10. Prover creates Proof for Proof Request\n')
        claim_for_attr_1 = claims_for_proof_request['attrs']['attr1_referent']
        referent = claim_for_attr_1[0]['referent']
        print_log('Referent: ')
        pprint.pprint(referent)
        requested_claims_json = json.dumps({
            'self_attested_attributes': {},
            'requested_attrs': {
                'attr1_referent': [referent, True]
            },
            'requested_predicates': {
                'predicate1_referent': referent
            }
        })
        pprint.pprint(json.loads(requested_claims_json))
        schemas_json = json.dumps({referent: schema})
        claim_defs_json = json.dumps({referent: json.loads(claim_def_json)})
        revoc_regs_json = json.dumps({})
        proof_json = await anoncreds.prover_create_proof(prover_wallet_handle, proof_req_json, requested_claims_json, schemas_json,
                                                         'master_secret', claim_defs_json, revoc_regs_json)
        proof = json.loads(proof_json)

        assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent'][1]

        # 11.
        print_log('\n11.Verifier is verifying proof from Prover\n')
        assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, claim_defs_json, revoc_regs_json)

        # 12
        print_log('\n12. Closing both wallet_handles\n')
        await wallet.close_wallet(issuer_wallet_handle)
        await wallet.close_wallet(prover_wallet_handle)

        # 13
        print_log('\n13. Deleting created wallet_handles\n')
        await wallet.delete_wallet(prover_wallet_name, None)
        await wallet.delete_wallet(issuer_wallet_name, None)

    except IndyError as e:
        print('Error occurred: %s' % e)


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(proof_negotiation())
    loop.close()


if __name__ == '__main__':
    main()

