from indy import anoncreds, wallet

from tests.utils import storage

import pytest
import logging
import json

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_anoncreds_demo_works(cleanup_storage):
    pool_name = "anoncreds_demo_pool"
    wallet_name = "anoncreds_demo_wallet"

    # 1. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    # 2. Issuer create Claim Definition for Schema
    schema_seq_no = 1
    schema = {
        'seqNo': schema_seq_no,
        'data': {
            'name': 'gvt',
            'version': '1.0',
            'keys': ['age', 'sex', 'height', 'name']
        }
    }
    schema_json = json.dumps(schema)
    issuer_did = 'NcYxiDXkpYi6ov5FcYDi1e'

    claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(wallet_handle, issuer_did, schema_json, 'CL', False)

    # 3. Prover create Master Secret
    master_secret_name = 'master_secret'
    await anoncreds.prover_create_master_secret(wallet_handle, master_secret_name)

    # 4. Prover create Claim Request
    prover_did = 'BzfFCYk'
    claim_offer = {
        'issuer_did': issuer_did,
        'schema_seq_no': schema_seq_no
    }

    claim_req_json = await anoncreds.prover_create_and_store_claim_req(wallet_handle, prover_did,
                                                                       json.dumps(claim_offer),
                                                                       claim_def_json, master_secret_name)

    # 5. Issuer create Claim for Claim Request
    claim = {
        'sex': ['male', '5944657099558967239210949258394887428692050081607692519917050011144233115103'],
        'name': ['Alex', '1139481716457488690172217916278103335'],
        'height': ['175', '175'],
        'age': ['28', '28']
    }

    (_, claim_json) = await anoncreds.issuer_create_claim(wallet_handle, claim_req_json, json.dumps(claim), -1, -1)

    # 6. Prover process and store Claim
    await anoncreds.prover_store_claim(wallet_handle, claim_json)

    # 7. Prover gets Claims for Proof Request
    proof_req = {
        'nonce': '123432421212',
        'name': 'proof_req_1',
        'version': '0.1',
        'requested_attrs': {
            'attr1_uuid': {'schema_seq_no': schema_seq_no, 'name': 'name'}
        },
        'requested_predicates': {
            'predicate1_uuid': {'attr_name': 'age', 'p_type': 'GE', 'value': 18}
        }
    }
    proof_req_json = json.dumps(proof_req)

    claim_for_proof_json = await anoncreds.prover_get_claims_for_proof_req(wallet_handle, proof_req_json)
    claims_for_proof = json.loads(claim_for_proof_json)

    claim_for_attr1 = claims_for_proof['attrs']['attr1_uuid']
    assert 1 == len(claim_for_attr1)

    claim_uuid = claim_for_attr1[0]['claim_uuid']

    # 8. Prover create Proof for Proof Request
    requested_claims = {
        'self_attested_attributes': {},
        'requested_attrs': {'attr1_uuid': [claim_uuid, True]},
        'requested_predicates': {'predicate1_uuid': claim_uuid}
    }
    requested_claims_json = json.dumps(requested_claims)

    schemas_json = json.dumps({claim_uuid: schema})
    claim_defs_json = json.dumps({claim_uuid: json.loads(claim_def_json)})
    revoc_regs_json = json.dumps({})

    proof_json = await anoncreds.prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json,
                                                     master_secret_name, claim_defs_json, revoc_regs_json)
    proof = json.loads(proof_json)

    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_uuid'][1]

    # 9. Verifier verify proof
    assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, claim_defs_json,
                                                 revoc_regs_json)

    # 10. Close wallet
    await wallet.close_wallet(wallet_handle)
