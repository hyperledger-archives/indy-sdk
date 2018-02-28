from indy import anoncreds, wallet

import json

import logging

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


async def demo():
    logger.info("Anoncreds sample -> started")

    pool_name = 'pool1'
    issuer_wallet_name = 'issuer_wallet'
    prover_wallet_name = 'prover_wallet'
    seq_no = 1
    issuer_did = 'NcYxiDXkpYi6ov5FcYDi1e'
    prover_did = 'VsKV7grR1BUE29mG2Fm2kX'

    # 1. Create Issuer Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, issuer_wallet_name, None, None, None)
    issuer_wallet = await wallet.open_wallet(issuer_wallet_name, None, None)

    # 2. Create Prover Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, prover_wallet_name, None, None, None)
    prover_wallet = await wallet.open_wallet(prover_wallet_name, None, None)

    # 3. Issuer create Claim Definition for Schema
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

    claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(issuer_wallet, issuer_did, schema_json, 'CL', False)

    # 4. Prover create Master Secret
    master_secret_name = 'master_secret'
    await anoncreds.prover_create_master_secret(prover_wallet, master_secret_name)

    # 5. Issuer create Claim Offer
    claim_offer_json = await anoncreds.issuer_create_claim_offer(issuer_wallet, schema_json, issuer_did, prover_did)

    # 6. Prover create Claim Request
    claim_req_json = await anoncreds.prover_create_and_store_claim_req(prover_wallet, prover_did, claim_offer_json,
                                                                       claim_def_json, master_secret_name)

    # 7. Issuer create Claim for Claim Request
    claim_json = json.dumps({
        'sex': ['male', '5944657099558967239210949258394887428692050081607692519917050011144233115103'],
        'name': ['Alex', '1139481716457488690172217916278103335'],
        'height': ['175', '175'],
        'age': ['28', '28']
    })

    (_, claim_json) = await anoncreds.issuer_create_claim(issuer_wallet, claim_req_json, claim_json, -1)

    # 8. Prover process and store Claim
    await anoncreds.prover_store_claim(prover_wallet, claim_json, None)

    # 9. Prover gets Claims for Proof Request
    proof_req_json = json.dumps({
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
    })

    claim_for_proof_request_json = await anoncreds.prover_get_claims_for_proof_req(prover_wallet, proof_req_json)
    claims_for_proof = json.loads(claim_for_proof_request_json)

    claim_for_attr1 = claims_for_proof['attrs']['attr1_referent']
    referent = claim_for_attr1[0]['referent']

    # 10. Prover create Proof for Proof Request
    requested_claims_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attrs': {'attr1_referent': [referent, True]},
        'requested_predicates': {'predicate1_referent': referent}
    })

    schemas_json = json.dumps({referent: schema})
    claim_defs_json = json.dumps({referent: json.loads(claim_def_json)})
    revoc_regs_json = json.dumps({})

    proof_json = await anoncreds.prover_create_proof(prover_wallet, proof_req_json, requested_claims_json, schemas_json,
                                                     'master_secret', claim_defs_json, revoc_regs_json)
    proof = json.loads(proof_json)

    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent'][1]

    # 11. Verifier verify proof
    assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, claim_defs_json,
                                                 revoc_regs_json)

    # 12. Close and delete Issuer wallet
    await wallet.close_wallet(issuer_wallet)
    await wallet.delete_wallet(issuer_wallet_name, None)

    # 13. Close and delete Prover wallet
    await wallet.close_wallet(prover_wallet)
    await wallet.delete_wallet(prover_wallet_name, None)

    logger.info("Anoncreds sample -> completed")
