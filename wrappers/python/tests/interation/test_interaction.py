import time

from indy import anoncreds, wallet, blob_storage

import pytest
import json

from indy import ledger
from tests.ledger.test_submit_request import ensure_previous_request_applied


@pytest.mark.asyncio
async def test_anoncreds_revocation_interaction_test_issuance_by_demand(pool_name, pool_handle, wallet_handle,
                                                                        identity_my, identity_my1, path_home, did_my2,
                                                                        credentials):
    issuer_did, _ = identity_my
    issuer_wallet_handle = wallet_handle

    prover_did, _ = identity_my1

    #  Prover Creates Wallet and Get Wallet Handle
    prover_wallet_config = '{"id":"prover_wallet"}'
    await wallet.create_wallet(prover_wallet_config, credentials)
    prover_wallet_handle = await wallet.open_wallet(prover_wallet_config, credentials)

    # Issuer Creates Schema
    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(issuer_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))

    # Issuer Posts Schema
    schema_request = await ledger.build_schema_request(issuer_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, schema_request)

    # Issuer Gets Schema from Ledger
    get_schema_request = await ledger.build_get_schema_request(issuer_did, str(schema_id))
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    #  Issuer Creates credential Definition for Schema
    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(issuer_wallet_handle, issuer_did, schema_json,
                                                               'tag1', 'CL', '{"support_revocation": true}')

    # Issuer Posts Credential Definition
    cred_def_request = await ledger.build_cred_def_request(issuer_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, cred_def_request)

    #  Issuer Creates Revocation Registry
    tails_writer_config = json.dumps({'base_dir': str(path_home.joinpath("tails")), 'uri_pattern': ''})
    tails_writer = await blob_storage.open_writer('default', tails_writer_config)
    (rev_reg_def_id, rev_reg_def_json, rev_reg_entry_json) = \
        await anoncreds.issuer_create_and_store_revoc_reg(issuer_wallet_handle, issuer_did, None, 'tag1', cred_def_id,
                                                          '{"max_cred_num": 5, "issuance_type":"ISSUANCE_ON_DEMAND"}',
                                                          tails_writer)

    # Issuer posts Revocation Registry Definition to Ledger
    revoc_reg_request = await ledger.build_revoc_reg_def_request(issuer_did, rev_reg_def_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_request)

    # Issuer posts Revocation Registry Entry to Ledger
    revoc_reg_entry_request = \
        await ledger.build_revoc_reg_entry_request(issuer_did, rev_reg_def_id, "CL_ACCUM", rev_reg_entry_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_entry_request)

    # ISSUANCE Credential for Prover

    # Prover Creates Master Secret
    master_secret_id = "master_secret"
    await anoncreds.prover_create_master_secret(prover_wallet_handle, master_secret_id)

    #  Issuer Creates credential Offer
    cred_offer_json = await anoncreds.issuer_create_credential_offer(issuer_wallet_handle, cred_def_id)
    cred_offer = json.loads(cred_offer_json)

    # Prover Gets Credential Definition from Ledger
    get_cred_def_request = await ledger.build_get_cred_def_request(prover_did, cred_offer['cred_def_id'])
    get_cred_def_response = \
        await ensure_previous_request_applied(pool_handle, get_cred_def_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (cred_def_id, cred_def_json) = await ledger.parse_get_cred_def_response(get_cred_def_response)

    #  Prover create credential Request
    (cred_req_json, cred_req_metadata_json) = \
        await anoncreds.prover_create_credential_req(prover_wallet_handle, prover_did, cred_offer_json,
                                                     cred_def_json, master_secret_id)

    #  Issuer Opens Tails reader
    blob_storage_reader_cfg_handle = await blob_storage.open_reader('default', tails_writer_config)

    #  Issuer create credential for credential Request
    #  note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
    cred_values_json = json.dumps({
        "sex": {
            "raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    })

    (cred_json, cred_rev_id, rev_reg_delta_json) = \
        await anoncreds.issuer_create_credential(issuer_wallet_handle, cred_offer_json, cred_req_json,
                                                 cred_values_json, rev_reg_def_id, blob_storage_reader_cfg_handle)

    # Issuer Posts Revocation Registry Delta to Ledger
    revoc_reg_entry_request = \
        await ledger.build_revoc_reg_entry_request(issuer_did, rev_reg_def_id, "CL_ACCUM", rev_reg_delta_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_entry_request)

    # Prover Gets RevocationRegistryDefinition
    credential = json.loads(cred_json)
    get_revoc_reg_def_request = await ledger.build_get_revoc_reg_def_request(prover_did, credential['rev_reg_id'])
    get_revoc_reg_def_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_def_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (rev_reg_id, revoc_reg_def_json) = await ledger.parse_get_revoc_reg_def_response(get_revoc_reg_def_response)

    # Prover Stores Credential
    cred_id = 'cred_1_id'
    await anoncreds.prover_store_credential(prover_wallet_handle, cred_id, cred_req_metadata_json,
                                            cred_json, cred_def_json, revoc_reg_def_json)

    #  VERIFYING Prover Credential
    time.sleep(2)
    to = int(time.time())

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
        "non_revoked": {"to": to}
    })

    # Prover Gets credentials for Proof Request
    search_credentials_for_proof_handle = \
        await anoncreds.prover_search_credentials_for_proof_req(prover_wallet_handle, proof_req_json, None)
    fetched_credential_json = \
        await anoncreds.prover_fetch_credentials_for_proof_req(search_credentials_for_proof_handle,
                                                               'attr1_referent', 10)
    await anoncreds.prover_close_credentials_search_for_proof_req(search_credentials_for_proof_handle)
    cred_info = json.loads(fetched_credential_json)[0]['cred_info']

    # Prover Gets RevocationRegistryDelta from Ledger
    get_revoc_reg_delta_request = await ledger.build_get_revoc_reg_delta_request(prover_did, rev_reg_def_id, None, to)
    get_revoc_reg_delta_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_delta_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (rev_reg_id, revoc_reg_delta_json, timestamp) = \
        await ledger.parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response)

    # Prover Creates Revocation State
    rev_state_json = await anoncreds.create_revocation_state(blob_storage_reader_cfg_handle, revoc_reg_def_json,
                                                             revoc_reg_delta_json, timestamp, cred_info['cred_rev_id'])

    # Prover Gets Schema from Ledger
    get_schema_request = await ledger.build_get_schema_request(prover_did, str(cred_info["schema_id"]))
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    # Prover Creates Proof for Proof Request
    requested_credentials_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {'attr1_referent':
                                     {'cred_id': cred_info['referent'], 'revealed': True, 'timestamp': timestamp}},
        'requested_predicates': {'predicate1_referent': {'cred_id': cred_info['referent'], 'timestamp': timestamp}}
    })

    schemas_json = json.dumps({schema_id: json.loads(schema_json)})
    credential_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
    revoc_states_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_state_json)}})

    proof_json = await anoncreds.prover_create_proof(prover_wallet_handle, proof_req_json, requested_credentials_json,
                                                     master_secret_id, schemas_json, credential_defs_json,
                                                     revoc_states_json)
    proof = json.loads(proof_json)

    # Verifier Gets required entities from Ledger
    verifier_did = did_my2
    identifier = proof['identifiers'][0]

    # Verifier Gets Schema from Ledger
    get_schema_request = await ledger.build_get_schema_request(verifier_did, identifier['schema_id'])
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    # Verifier Gets Credential Definition from Ledger
    get_cred_def_request = await ledger.build_get_cred_def_request(verifier_did, identifier['cred_def_id'])
    get_cred_def_response = \
        await ensure_previous_request_applied(pool_handle, get_cred_def_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (cred_def_id, cred_def_json) = await ledger.parse_get_cred_def_response(get_cred_def_response)

    # Verifier Gets Revocation Registry Definition from Ledger
    get_revoc_reg_def_request = await ledger.build_get_revoc_reg_def_request(verifier_did, identifier['rev_reg_id'])
    get_revoc_reg_def_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_def_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (rev_reg_id, revoc_reg_def_json) = await ledger.parse_get_revoc_reg_def_response(get_revoc_reg_def_response)

    # Verifier Gets Revocation Registry from Ledger
    get_revoc_reg_request = \
        await ledger.build_get_revoc_reg_request(verifier_did, identifier['rev_reg_id'], identifier['timestamp'])
    get_revoc_reg_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (rev_reg_id, rev_reg_json, identifier) = await ledger.parse_get_revoc_reg_response(get_revoc_reg_response)

    # Verifier verify proof
    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']

    schemas_json = json.dumps({schema_id: json.loads(schema_json)})
    credential_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
    revoc_ref_defs_json = json.dumps({rev_reg_id: json.loads(revoc_reg_def_json)})
    revoc_regs_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_reg_json)}})

    assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, credential_defs_json,
                                                 revoc_ref_defs_json, revoc_regs_json)

    #  Issuer revokes cred_info
    rev_reg_delta_json = await anoncreds.issuer_revoke_credential(issuer_wallet_handle, blob_storage_reader_cfg_handle,
                                                                  rev_reg_def_id, cred_rev_id)

    # Issuer Posts RevocationRegistryDelta to Ledger
    revoc_reg_entry_request = \
        await ledger.build_revoc_reg_entry_request(issuer_did, rev_reg_def_id, "CL_ACCUM", rev_reg_delta_json)
    await ledger.sign_and_submit_request(pool_handle, issuer_wallet_handle, issuer_did, revoc_reg_entry_request)

    #  VERIFYING Prover Credential after Revocation
    time.sleep(2)
    from_ = to
    to = int(time.time())

    # Prover Gets RevocationRegistryDelta from Ledger
    get_revoc_reg_delta_request = await ledger.build_get_revoc_reg_delta_request(prover_did, rev_reg_def_id, from_, to)
    get_revoc_reg_delta_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_delta_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (rev_reg_id, revoc_reg_delta_json, identifier) = \
        await ledger.parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response)

    # Prover Creates Revocation State
    rev_state_json = await anoncreds.create_revocation_state(blob_storage_reader_cfg_handle, revoc_reg_def_json,
                                                             revoc_reg_delta_json, timestamp, cred_rev_id)

    # Prover Creates Proof for Proof Request
    requested_credentials_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {'attr1_referent':
                                     {'cred_id': cred_info['referent'], 'revealed': True, 'timestamp': timestamp}},
        'requested_predicates': {'predicate1_referent': {'cred_id': cred_info['referent'], 'timestamp': timestamp}}
    })
    revoc_states_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_state_json)}})

    proof_json = await anoncreds.prover_create_proof(prover_wallet_handle, proof_req_json, requested_credentials_json,
                                                     master_secret_id, schemas_json, credential_defs_json,
                                                     revoc_states_json)
    proof = json.loads(proof_json)
    identifier = proof['identifiers'][0]

    # Verifier Gets RevocationRegistry from Ledger
    get_revoc_reg_request = \
        await ledger.build_get_revoc_reg_request(verifier_did, identifier['rev_reg_id'], identifier['timestamp'])
    get_revoc_reg_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (rev_reg_id, rev_reg_json, timestamp) = await ledger.parse_get_revoc_reg_response(get_revoc_reg_response)

    revoc_regs_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_reg_json)}})

    assert not await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, credential_defs_json,
                                                     revoc_ref_defs_json, revoc_regs_json)

    #  Close and Delete Prover Wallet
    await wallet.close_wallet(prover_wallet_handle)
    await wallet.delete_wallet(prover_wallet_config, credentials)
