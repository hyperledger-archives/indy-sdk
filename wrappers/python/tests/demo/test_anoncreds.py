from indy import anoncreds, wallet

import pytest
import json


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_anoncreds_demo_works(pool_name, wallet_name, path_home):
    # 1. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    # 2. Issuer create Claim Definition for Schema
    schema = {
        'seqNo': 1,
        'data': {
            'name': 'gvt',
            'version': '1.0',
            'keys': ['age', 'sex', 'height', 'name']
        }
    }
    schema_json = json.dumps(schema)

    claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(wallet_handle,
                                                          'NcYxiDXkpYi6ov5FcYDi1e', schema_json, 'CL', False)

    # 3. Prover create Master Secret
    await anoncreds.prover_create_master_secret(wallet_handle, 'master_secret')

    # 4. Prover create Claim Request
    claim_offer_json = json.dumps({
        'issuer_did': 'NcYxiDXkpYi6ov5FcYDi1e',
        'schema_seq_no': 1
    })

    claim_req_json = await anoncreds.prover_create_and_store_claim_req(wallet_handle, 'BzfFCYk', claim_offer_json,
                                                                       claim_def_json, 'master_secret')

    # 5. Issuer create Claim for Claim Request
    claim_json = json.dumps({
        'sex': ['male', '5944657099558967239210949258394887428692050081607692519917050011144233115103'],
        'name': ['Alex', '1139481716457488690172217916278103335'],
        'height': ['175', '175'],
        'age': ['28', '28']
    })

    (_, claim_json) = await anoncreds.issuer_create_claim(wallet_handle, claim_req_json, claim_json, -1)

    # 6. Prover process and store Claim
    await anoncreds.prover_store_claim(wallet_handle, claim_json)

    # 7. Prover gets Claims for Proof Request
    proof_req_json = json.dumps({
        'nonce': '123432421212',
        'name': 'proof_req_1',
        'version': '0.1',
        'requested_attrs': {
            'attr1_uuid': {'schema_seq_no': 1, 'name': 'name'}
        },
        'requested_predicates': {
            'predicate1_uuid': {'attr_name': 'age', 'p_type': 'GE', 'value': 18}
        }
    })

    claim_for_proof_json = await anoncreds.prover_get_claims_for_proof_req(wallet_handle, proof_req_json)
    claims_for_proof = json.loads(claim_for_proof_json)

    claim_for_attr1 = claims_for_proof['attrs']['attr1_uuid']
    claim_uuid = claim_for_attr1[0]['claim_uuid']

    # 8. Prover create Proof for Proof Request
    requested_claims_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attrs': {'attr1_uuid': [claim_uuid, True]},
        'requested_predicates': {'predicate1_uuid': claim_uuid}
    })

    schemas_json = json.dumps({claim_uuid: schema})
    claim_defs_json = json.dumps({claim_uuid: json.loads(claim_def_json)})
    revoc_regs_json = json.dumps({})

    proof_json = await anoncreds.prover_create_proof(wallet_handle, proof_req_json, requested_claims_json, schemas_json,
                                                     'master_secret', claim_defs_json, revoc_regs_json)
    proof = json.loads(proof_json)

    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_uuid'][1]

    # 9. Verifier verify proof
    assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, claim_defs_json,
                                                 revoc_regs_json)

    # 10. Close wallet
    await wallet.close_wallet(wallet_handle)
