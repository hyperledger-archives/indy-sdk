from indy import anoncreds, wallet, blob_storage

import pytest
import json


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_anoncreds_demo_works(pool_name, wallet_name, path_home):
    # 1. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    # 2. Issuer create credential Definition for Schema
    issuer_did = 'NcYxiDXkpYi6ov5FcYDi1e'
    prover_did = 'VsKV7grR1BUE29mG2Fm2kX'

    (schema_id, schema_json) = await anoncreds.issuer_create_schema(issuer_did, "gvt", '1.0',
                                                                    '["age", "sex", "height", "name"]')

    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, issuer_did, schema_json, 'tag1', 'CL',
                                                               '{"support_revocation": false}')

    # 3. Prover create Master Secret
    master_secret_id = "master_secret"
    await anoncreds.prover_create_master_secret(wallet_handle, master_secret_id)

    # 4. Issuer create credential Offer
    cred_offer_json = await anoncreds.issuer_create_credential_offer(wallet_handle, cred_def_id)

    # 5. Prover create credential Request
    (cred_req_json, cred_req_metadata_json) = \
        await anoncreds.prover_create_credential_req(wallet_handle, prover_did, cred_offer_json,
                                                     cred_def_json, master_secret_id)

    #  6. Issuer create credential for credential Request
    cred_values_json = json.dumps({
        "sex": {
            "raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    })

    (cred_json, _, _) = await anoncreds.issuer_create_credential(wallet_handle, cred_offer_json, cred_req_json,
                                                                 cred_values_json, None, None)

    # 7. Prover process and store credential
    cred_id = 'cred_id_1'
    await anoncreds.prover_store_credential(wallet_handle, cred_id, cred_req_json, cred_req_metadata_json,
                                            cred_json, cred_def_json, None)

    # 8. Prover gets credentials for Proof Request
    proof_req_json = json.dumps({
        'nonce': '123432421212',
        'name': 'proof_req_1',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {'name': 'name'}
        },
        'requested_predicates': {
            'predicate1_referent': {'name': 'age', 'p_type': '>=', 'p_value': 18}
        }
    })

    credential_for_proof_json = await anoncreds.prover_get_credentials_for_proof_req(wallet_handle, proof_req_json)
    credentials_for_proof = json.loads(credential_for_proof_json)

    credential_for_attr1 = credentials_for_proof['attrs']['attr1_referent']
    referent = credential_for_attr1[0]['cred_info']['referent']

    # 9. Prover create Proof for Proof Request
    requested_credentials_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {'attr1_referent': {'cred_id': referent, 'revealed': True}},
        'requested_predicates': {'predicate1_referent': {'cred_id': referent}}
    })

    schemas_json = json.dumps({schema_id: json.loads(schema_json)})
    credential_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
    revoc_states_json = "{}"

    proof_json = await anoncreds.prover_create_proof(wallet_handle, proof_req_json, requested_credentials_json,
                                                     master_secret_id, schemas_json, credential_defs_json,
                                                     revoc_states_json)
    proof = json.loads(proof_json)

    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']

    # 10. Verifier verify proof
    revoc_ref_defs_json = "{}"
    revoc_regs_json = "{}"

    assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, credential_defs_json,
                                                 revoc_ref_defs_json, revoc_regs_json)

    # 11. Close wallet
    await wallet.close_wallet(wallet_handle)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_anoncreds_demo_works_for_revocation_proof(pool_name, wallet_name, path_home):
    # 1. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    issuer_did = 'NcYxiDXkpYi6ov5FcYDi1e'
    prover_did = 'VsKV7grR1BUE29mG2Fm2kX'

    # 2. Issuer create credential Definition for Schema
    (schema_id, schema_json) = await anoncreds.issuer_create_schema(issuer_did, "gvt", '1.0',
                                                                    '["age", "sex", "height", "name"]')

    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, issuer_did, schema_json, 'tag1', 'CL',
                                                               '{"support_revocation": true}')

    # 3. Issuer create Revocation Registry
    tails_writer_config = json.dumps({'base_dir': str(path_home.joinpath("tails")), 'uri_pattern': ''})
    tails_writer = await blob_storage.open_writer('default', tails_writer_config)
    (rev_reg_id, rev_reg_def_json, _) = \
        await anoncreds.issuer_create_and_store_revoc_reg(wallet_handle, issuer_did, None, 'tag1', cred_def_id,
                                                          '{"max_cred_num": 5}', tails_writer)

    # 4. Prover create Master Secret
    master_secret_id = "master_secret"
    await anoncreds.prover_create_master_secret(wallet_handle, master_secret_id)

    # 5. Issuer create credential Offer
    cred_offer_json = await anoncreds.issuer_create_credential_offer(wallet_handle, cred_def_id)

    # 6. Prover create credential Request
    (cred_req_json, cred_req_metadata_json) = \
        await anoncreds.prover_create_credential_req(wallet_handle, prover_did, cred_offer_json,
                                                     cred_def_json, master_secret_id)

    # 7. Issuer open Tails reader
    blob_storage_reader_cfg_handle = await blob_storage.open_reader('default', tails_writer_config)

    #  8. Issuer create credential for credential Request
    cred_values_json = json.dumps({
        "sex": {"raw": "male", "encoded": "594465709955896723921094925839488742869205008160769251991705001"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    })

    (cred_json, rev_id, rev_reg_delta_json) = \
        await anoncreds.issuer_create_credential(wallet_handle, cred_offer_json, cred_req_json,
                                                 cred_values_json, rev_reg_id, blob_storage_reader_cfg_handle)

    # 9. Prover process and store credential
    cred_id = 'cred_1_id'
    await anoncreds.prover_store_credential(wallet_handle, cred_id, cred_req_json, cred_req_metadata_json,
                                            cred_json, cred_def_json, rev_reg_def_json)

    # 10. Prover gets credentials for Proof Request
    proof_req_json = json.dumps({
        'nonce': '123432421212',
        'name': 'proof_req_1',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {'name': 'name'}
        },
        'requested_predicates': {
            'predicate1_referent': {'name': 'age', 'p_type': '>=', 'p_value': 18}
        },
        "non_revoked": {"from": 80, "to": 100}
    })

    credential_for_proof_json = await anoncreds.prover_get_credentials_for_proof_req(wallet_handle, proof_req_json)
    credentials_for_proof = json.loads(credential_for_proof_json)

    # 11. Prover creates revocation state
    timestamp = 100
    rev_state_json = await anoncreds.create_revocation_state(blob_storage_reader_cfg_handle, rev_reg_def_json,
                                                             rev_reg_delta_json, timestamp, rev_id)

    credential_for_attr1 = credentials_for_proof['attrs']['attr1_referent']
    referent = credential_for_attr1[0]['cred_info']['referent']

    # 12. Prover create Proof for Proof Request
    requested_credentials_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {'attr1_referent': {'cred_id': referent, 'revealed': True, 'timestamp': timestamp}},
        'requested_predicates': {'predicate1_referent': {'cred_id': referent, 'timestamp': timestamp}}
    })

    schemas_json = json.dumps({schema_id: json.loads(schema_json)})
    credential_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
    revoc_states_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_state_json)}})

    proof_json = await anoncreds.prover_create_proof(wallet_handle, proof_req_json, requested_credentials_json,
                                                     master_secret_id, schemas_json, credential_defs_json,
                                                     revoc_states_json)
    proof = json.loads(proof_json)

    # 13. Verifier verify proof
    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']

    revoc_ref_defs_json = json.dumps({rev_reg_id: json.loads(rev_reg_def_json)})
    revoc_regs_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_reg_delta_json)}})

    assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, credential_defs_json,
                                                 revoc_ref_defs_json, revoc_regs_json)

    # 14. Close wallet
    await wallet.close_wallet(wallet_handle)
