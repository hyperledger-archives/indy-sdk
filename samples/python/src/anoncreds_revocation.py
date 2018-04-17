import time

from indy import anoncreds, wallet

import json

import logging

from indy import blob_storage

from src.utils import run_coroutine, path_home

logger = logging.getLogger(__name__)


async def demo():
    logger.info("Anoncreds Revocation sample -> started")

    pool_name = 'pool1'
    issuer_wallet_name = 'issuer_wallet'
    prover_wallet_name = 'prover_wallet'
    issuer_did = 'NcYxiDXkpYi6ov5FcYDi1e'
    prover_did = 'VsKV7grR1BUE29mG2Fm2kX'

    # 1. Create Issuer Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, issuer_wallet_name, None, None, None)
    issuer_wallet = await wallet.open_wallet(issuer_wallet_name, None, None)

    # 2. Create Prover Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, prover_wallet_name, None, None, None)
    prover_wallet = await wallet.open_wallet(prover_wallet_name, None, None)

    # 3. Issuer create Credential Schema
    schema_name = 'gvt'
    schema_version = '1.0'
    schema_attributes = '["age", "sex", "height", "name"]'
    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(issuer_did, schema_name, schema_version, schema_attributes)

    # 4. Issuer create Credential Definition for Schema
    cred_def_tag = 'cred_def_tag'
    cred_def_type = 'CL'
    cred_def_config = json.dumps({"support_revocation": True})
    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(issuer_wallet, issuer_did, schema_json, cred_def_tag,
                                                               cred_def_type, cred_def_config)

    # 5. Issuer create Revocation Registry
    tails_writer_config = json.dumps({'base_dir': str(path_home().joinpath("tails")), 'uri_pattern': ''})
    tails_writer = await blob_storage.open_writer('default', tails_writer_config)

    rvoc_reg_def_tag = 'cred_def_tag'
    rvoc_reg_def_config = json.dumps({"max_cred_num": 5, 'issuance_type': 'ISSUANCE_ON_DEMAND'})
    (rev_reg_id, rev_reg_def_json, _) = \
        await anoncreds.issuer_create_and_store_revoc_reg(issuer_wallet, issuer_did, None, rvoc_reg_def_tag,
                                                          cred_def_id, rvoc_reg_def_config, tails_writer)

    # 6. Prover create Master Secret
    master_secret_id = await anoncreds.prover_create_master_secret(prover_wallet, None)

    #  7. Issuer create Credential Offer
    cred_offer_json = await anoncreds.issuer_create_credential_offer(issuer_wallet, cred_def_id)

    # 8. Prover create Credential Request
    (cred_req_json, cred_req_metadata_json) = \
        await anoncreds.prover_create_credential_req(prover_wallet, prover_did, cred_offer_json,
                                                     cred_def_json, master_secret_id)

    # 9. Issuer open Tails reader
    blob_storage_reader_handle = await blob_storage.open_reader('default', tails_writer_config)

    # 10. Issuer create Credential
    cred_values_json = json.dumps({
        "sex": {"raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    })

    (cred_json, rev_id, rev_reg_delta_json) = \
        await anoncreds.issuer_create_credential(issuer_wallet, cred_offer_json, cred_req_json,
                                                 cred_values_json, rev_reg_id, blob_storage_reader_handle)

    # 11. Prover store Credential
    await anoncreds.prover_store_credential(prover_wallet, None, cred_req_json, cred_req_metadata_json,
                                            cred_json, cred_def_json, rev_reg_def_json)

    # 11. Prover gets Credentials for Proof Request
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

    credential_for_proof_json = await anoncreds.prover_get_credentials_for_proof_req(prover_wallet, proof_req_json)
    creds_for_proof = json.loads(credential_for_proof_json)

    cred_for_attr1 = creds_for_proof['attrs']['attr1_referent']
    cred_for_attr1_referent = cred_for_attr1[0]['cred_info']['referent']
    cred_for_predicate1 = creds_for_proof['predicates']['predicate1_referent']
    cred_for_predicate1_referent = cred_for_predicate1[0]['cred_info']['referent']

    # 12. Prover creates revocation state
    timestamp = 100
    rev_state_json = await anoncreds.create_revocation_state(blob_storage_reader_handle, rev_reg_def_json,
                                                             rev_reg_delta_json, timestamp, rev_id)

    # 13. Prover create Proof for Proof Request
    requested_credentials_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {'attr1_referent': {
            'cred_id': cred_for_attr1_referent, 'revealed': True, 'timestamp': timestamp}
        },
        'requested_predicates': {
            'predicate1_referent': {'cred_id': cred_for_predicate1_referent, 'timestamp': timestamp}
        }
    })

    schemas_json = json.dumps({schema_id: json.loads(schema_json)})
    cred_defs_json = json.dumps({cred_def_id: json.loads(cred_def_json)})
    revoc_states_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_state_json)}})

    proof_json = await anoncreds.prover_create_proof(prover_wallet, proof_req_json, requested_credentials_json,
                                                     master_secret_id, schemas_json, cred_defs_json, revoc_states_json)
    proof = json.loads(proof_json)

    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']

    # 12. Verifier verify proof
    revoc_ref_defs_json = json.dumps({rev_reg_id: json.loads(rev_reg_def_json)})
    revoc_regs_json = json.dumps({rev_reg_id: {timestamp: json.loads(rev_reg_delta_json)}})

    assert await anoncreds.verifier_verify_proof(proof_req_json, proof_json, schemas_json, cred_defs_json,
                                                 revoc_ref_defs_json, revoc_regs_json)

    # 13. Close and delete Issuer wallet
    await wallet.close_wallet(issuer_wallet)
    await wallet.delete_wallet(issuer_wallet_name, None)

    # 14. Close and delete Prover wallet
    await wallet.close_wallet(prover_wallet)
    await wallet.delete_wallet(prover_wallet_name, None)

    logger.info("Anoncreds Revocation sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
