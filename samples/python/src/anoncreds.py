import time

from indy import anoncreds, wallet

import json

import logging

from indy import pool

from src.utils import run_coroutine, PROTOCOL_VERSION

logger = logging.getLogger(__name__)

async def demo():
    logger.info("Anoncreds sample -> started")

    issuer = {
        'did': 'NcYxiDXkpYi6ov5FcYDi1e',
        'wallet_config': json.dumps({'id': 'issuer_wallet'}),
        'wallet_credentials': json.dumps({'key': 'issuer_wallet_key'})
    }
    prover = {
        'did': 'VsKV7grR1BUE29mG2Fm2kX',
        'wallet_config': json.dumps({"id": "prover_wallet"}),
        'wallet_credentials': json.dumps({"key": "issuer_wallet_key"})
    }
    verifier = {}
    store = {}

    # Set protocol version 2 to work with Indy Node 1.4
    await pool.set_protocol_version(PROTOCOL_VERSION)

    # 1. Create Issuer Wallet and Get Wallet Handle
    await wallet.create_wallet(issuer['wallet_config'], issuer['wallet_credentials'])
    issuer['wallet'] = await wallet.open_wallet(issuer['wallet_config'], issuer['wallet_credentials'])

    # 2. Create Prover Wallet and Get Wallet Handle
    await wallet.create_wallet(prover['wallet_config'], prover['wallet_credentials'])
    prover['wallet'] = await wallet.open_wallet(prover['wallet_config'], prover['wallet_credentials'])

    # 3. Issuer create Credential Schema
    schema = {
        'name': 'gvt',
        'version': '1.0',
        'attributes': '["age", "sex", "height", "name"]'
    }
    issuer['schema_id'], issuer['schema'] = await anoncreds.issuer_create_schema(issuer['did'], schema['name'],
                                                                                 schema['version'],
                                                                                 schema['attributes'])
    store[issuer['schema_id']] = issuer['schema']

    # 4. Issuer create Credential Definition for Schema
    cred_def = {
        'tag': 'cred_def_tag',
        'type': 'CL',
        'config': json.dumps({"support_revocation": False})
    }
    issuer['cred_def_id'], issuer['cred_def'] = await anoncreds.issuer_create_and_store_credential_def(
        issuer['wallet'], issuer['did'], issuer['schema'], cred_def['tag'], cred_def['type'], cred_def['config'])
    store[issuer['cred_def_id']] = issuer['cred_def']

    # 5. Prover create Master Secret
    prover['master_secret_id'] = await anoncreds.prover_create_master_secret(prover['wallet'], None)

    #  6. Issuer create Credential Offer
    issuer['cred_offer'] = await anoncreds.issuer_create_credential_offer(issuer['wallet'], issuer['cred_def_id'])
    prover['cred_offer'] = issuer['cred_offer']

    cred_offer = json.loads(prover['cred_offer'])
    prover['cred_def_id'] = cred_offer['cred_def_id']
    prover['schema_id'] = cred_offer['schema_id']

    prover['cred_def'] = store[prover['cred_def_id']]
    prover['schema'] = store[prover['schema_id']]

    # 7. Prover create Credential Request
    prover['cred_req'], prover['cred_req_metadata'] = \
        await anoncreds.prover_create_credential_req(prover['wallet'], prover['did'], prover['cred_offer'],
                                                     prover['cred_def'], prover['master_secret_id'])

    # 8. Issuer create Credential
    prover['cred_values'] = json.dumps({
        "sex": {"raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    })
    issuer['cred_values'] = prover['cred_values']
    issuer['cred_req'] = prover['cred_req']

    (cred_json, _, _) = await anoncreds.issuer_create_credential(issuer['wallet'], issuer['cred_offer'],
                                                                 issuer['cred_req'], issuer['cred_values'], None, None)
    prover['cred'] = cred_json

    # 9. Prover store Credential
    await anoncreds.prover_store_credential(prover['wallet'], None, prover['cred_req_metadata'], prover['cred'],
                                            prover['cred_def'], None)

    # 10. Prover gets Credentials for Proof Request
    verifier['proof_req'] = json.dumps({
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
    prover['proof_req'] = verifier['proof_req']

    # Prover gets Credentials for attr1_referent
    prover['cred_search_handle'] = \
        await anoncreds.prover_search_credentials_for_proof_req(prover['wallet'], prover['proof_req'], None)

    creds_for_attr1 = await anoncreds.prover_fetch_credentials_for_proof_req(prover['cred_search_handle'],
                                                                             'attr1_referent', 10)
    prover['cred_for_attr1'] = json.loads(creds_for_attr1)[0]['cred_info']

    # Prover gets Credentials for predicate1_referent
    creds_for_predicate1 = await anoncreds.prover_fetch_credentials_for_proof_req(prover['cred_search_handle'],
                                                                                  'predicate1_referent', 10)
    prover['cred_for_predicate1'] = json.loads(creds_for_predicate1)[0]['cred_info']

    await anoncreds.prover_close_credentials_search_for_proof_req(prover['cred_search_handle'])

    # 11. Prover create Proof for Proof Request
    prover['requested_creds'] = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {'attr1_referent': {'cred_id': prover['cred_for_attr1']['referent'], 'revealed': True}},
        'requested_predicates': {'predicate1_referent': {'cred_id': prover['cred_for_predicate1']['referent']}}
    })

    schemas_json = json.dumps({prover['schema_id']: json.loads(prover['schema'])})
    cred_defs_json = json.dumps({prover['cred_def_id']: json.loads(prover['cred_def'])})
    revoc_states_json = json.dumps({})

    prover['proof'] = await anoncreds.prover_create_proof(prover['wallet'], prover['proof_req'],
                                                          prover['requested_creds'],
                                                          prover['master_secret_id'], schemas_json, cred_defs_json,
                                                          revoc_states_json)
    verifier['proof'] = prover['proof']

    # 12. Verifier verify proof
    proof = json.loads(verifier['proof'])
    assert 'Alex' == proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']

    identifier = proof['identifiers'][0]

    verifier['cred_def_id'] = identifier['cred_def_id']
    verifier['schema_id'] = identifier['schema_id']

    verifier['cred_def'] = store[verifier['cred_def_id']]
    verifier['schema'] = store[verifier['schema_id']]

    schemas_json = json.dumps({verifier['schema_id']: json.loads(verifier['schema'])})
    cred_defs_json = json.dumps({verifier['cred_def_id']: json.loads(verifier['cred_def'])})
    revoc_ref_defs_json = "{}"
    revoc_regs_json = "{}"

    assert await anoncreds.verifier_verify_proof(verifier['proof_req'], verifier['proof'], schemas_json, cred_defs_json,
                                                 revoc_ref_defs_json, revoc_regs_json)

    # 13. Close and delete Issuer wallet
    await wallet.close_wallet(issuer['wallet'])
    await wallet.delete_wallet(issuer['wallet_config'], issuer['wallet_credentials'])

    # 14. Close and delete Prover wallet
    await wallet.close_wallet(prover['wallet'])
    await wallet.delete_wallet(prover['wallet_config'], prover['wallet_credentials'])

    logger.info("Anoncreds sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
