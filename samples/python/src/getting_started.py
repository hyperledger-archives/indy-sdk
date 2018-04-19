import time

from indy import anoncreds, crypto, did, ledger, pool, wallet

import json
import logging
from typing import Optional

from indy.error import ErrorCode, IndyError

from src.utils import get_pool_genesis_txn_path, run_coroutine

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


async def run():
    logger.info("Getting started -> started")

    pool_name = 'pool1'
    logger.info("Open Pool Ledger: {}".format(pool_name))
    pool_genesis_txn_path = get_pool_genesis_txn_path(pool_name)
    pool_config = json.dumps({"genesis_txn": str(pool_genesis_txn_path)})
    try:
        await pool.create_pool_ledger_config(pool_name, pool_config)
    except IndyError as ex:
        if ex.error_code == ErrorCode.PoolLedgerConfigAlreadyExistsError:
            pass
    pool_handle = await pool.open_pool_ledger(pool_name, None)

    logger.info("==============================")
    logger.info("=== Getting Trust Anchor credentials for Faber, Acme, Thrift and Government  ==")
    logger.info("------------------------------")

    logger.info("\"Sovrin Steward\" -> Create wallet")
    steward_wallet_name = 'sovrin_steward_wallet'
    try:
        await wallet.create_wallet(pool_name, steward_wallet_name, None, None,
                                   None)
    except IndyError as ex:
        if ex.error_code == ErrorCode.WalletAlreadyExistsError:
            pass

    steward_wallet = await wallet.open_wallet(steward_wallet_name, None, None)

    logger.info("\"Sovrin Steward\" -> Create and store in Wallet DID from seed")
    steward_did_info = {'seed': '000000000000000000000000Steward1'}
    (steward_did, steward_key) = await did.create_and_store_my_did(steward_wallet, json.dumps(steward_did_info))

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Government Onboarding  ==")
    logger.info("------------------------------")

    government_wallet, government_wallet_name, steward_government_key, government_steward_did, government_steward_key, _ \
        = await onboarding(pool_handle, pool_name, "Sovrin Steward", steward_wallet,
                           steward_did, "Government", None, 'government_wallet')

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Government getting Verinym  ==")
    logger.info("------------------------------")

    government_did = await get_verinym(pool_handle, "Sovrin Steward", steward_wallet, steward_did,
                                       steward_government_key, "Government", government_wallet, government_steward_did,
                                       government_steward_key, 'TRUST_ANCHOR')

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Faber Onboarding  ==")
    logger.info("------------------------------")
    faber_wallet, faber_wallet_name, steward_faber_key, faber_steward_did, faber_steward_key, _ = \
        await onboarding(pool_handle, pool_name, "Sovrin Steward", steward_wallet, steward_did,
                         "Faber", None, 'faber_wallet')

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Faber getting Verinym  ==")
    logger.info("------------------------------")

    faber_did = await get_verinym(pool_handle, "Sovrin Steward", steward_wallet, steward_did, steward_faber_key,
                                  "Faber", faber_wallet, faber_steward_did, faber_steward_key, 'TRUST_ANCHOR')

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Acme Onboarding  ==")
    logger.info("------------------------------")

    acme_wallet, acme_wallet_name, steward_acme_key, acme_steward_did, acme_steward_key, _ = \
        await onboarding(pool_handle, pool_name, "Sovrin Steward", steward_wallet, steward_did,
                         "Acme", None, 'acme_wallet')

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Acme getting Verinym  ==")
    logger.info("------------------------------")

    acme_did = await get_verinym(pool_handle, "Sovrin Steward", steward_wallet, steward_did, steward_acme_key,
                                 "Acme", acme_wallet, acme_steward_did, acme_steward_key, 'TRUST_ANCHOR')

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Thrift Onboarding  ==")
    logger.info("------------------------------")

    thrift_wallet, thrift_wallet_name, steward_thrift_key, thrift_steward_did, thrift_steward_key, _ = \
        await onboarding(pool_handle, pool_name, "Sovrin Steward", steward_wallet, steward_did,
                         "Thrift", None, ' thrift_wallet')

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Thrift getting Verinym  ==")
    logger.info("------------------------------")

    thrift_did = await get_verinym(pool_handle, "Sovrin Steward", steward_wallet, steward_did, steward_thrift_key,
                                   "Thrift", thrift_wallet, thrift_steward_did, thrift_steward_key, 'TRUST_ANCHOR')

    logger.info("==============================")
    logger.info("=== Credential Schemas Setup ==")
    logger.info("------------------------------")

    logger.info("\"Government\" -> Create \"Job-Certificate\" Schema")
    (job_certificate_schema_id, job_certificate_schema) = \
        await anoncreds.issuer_create_schema(government_did, 'Job-Certificate', '0.2',
                                             json.dumps(['first_name', 'last_name', 'salary', 'employee_status',
                                                         'experience']))

    logger.info("\"Government\" -> Send \"Job-Certificate\" Schema to Ledger")
    await send_schema(pool_handle, government_wallet, government_did, job_certificate_schema)

    logger.info("\"Government\" -> Create \"Transcript\" Schema")
    (transcript_schema_id, transcript_schema) = \
        await anoncreds.issuer_create_schema(government_did, 'Transcript', '1.2',
                                             json.dumps(['first_name', 'last_name', 'degree', 'status',
                                                         'year', 'average', 'ssn']))
    logger.info("\"Government\" -> Send \"Transcript\" Schema to Ledger")
    await send_schema(pool_handle, government_wallet, government_did, transcript_schema)

    logger.info("==============================")
    logger.info("=== Faber Credential Definition Setup ==")
    logger.info("------------------------------")

    logger.info("\"Faber\" -> Get \"Transcript\" Schema from Ledger")
    (_, transcript_schema) = await get_schema(pool_handle, faber_did, transcript_schema_id)

    logger.info("\"Faber\" -> Create and store in Wallet \"Faber Transcript\" Credential Definition")
    (faber_transcript_cred_def_id, faber_transcript_cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(faber_wallet, faber_did, transcript_schema,
                                                               'TAG1', 'CL', '{"support_revocation": false}')

    logger.info("\"Faber\" -> Send  \"Faber Transcript\" Credential Definition to Ledger")
    await send_cred_def(pool_handle, faber_wallet, faber_did, faber_transcript_cred_def_json)

    logger.info("==============================")
    logger.info("=== Acme Credential Definition Setup ==")
    logger.info("------------------------------")

    logger.info("\"Acme\" ->  Get from Ledger \"Job-Certificate\" Schema")
    (_, job_certificate_schema) = await get_schema(pool_handle, acme_did, job_certificate_schema_id)

    logger.info("\"Acme\" -> Create and store in Wallet \"Acme Job-Certificate\" Credential Definition")
    (acme_job_certificate_cred_def_id, acme_job_certificate_cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(acme_wallet, acme_did, job_certificate_schema,
                                                               'TAG1', 'CL', '{"support_revocation": false}')

    logger.info("\"Acme\" -> Send \"Acme Job-Certificate\" Credential Definition to Ledger")
    await send_cred_def(pool_handle, acme_wallet, acme_did, acme_job_certificate_cred_def_json)

    logger.info("==============================")
    logger.info("=== Getting Transcript with Faber ==")
    logger.info("==============================")
    logger.info("== Getting Transcript with Faber - Onboarding ==")
    logger.info("------------------------------")

    alice_wallet, alice_wallet_name, faber_alice_key, alice_faber_did, alice_faber_key, faber_alice_connection_response \
        = await onboarding(pool_handle, pool_name, "Faber", faber_wallet, faber_did, "Alice", None, ' alice_wallet')

    logger.info("==============================")
    logger.info("== Getting Transcript with Faber - Getting Transcript Credential ==")
    logger.info("------------------------------")

    logger.info("\"Faber\" -> Create \"Transcript\" Credential Offer for Alice")
    transcript_cred_offer_json = \
        await anoncreds.issuer_create_credential_offer(faber_wallet, faber_transcript_cred_def_id)

    logger.info("\"Faber\" -> Get key for Alice did")
    alice_faber_verkey = await did.key_for_did(pool_handle, acme_wallet, faber_alice_connection_response['did'])

    logger.info("\"Faber\" -> Authcrypt \"Transcript\" Credential Offer for Alice")
    authcrypted_transcript_cred_offer = await crypto.auth_crypt(faber_wallet, faber_alice_key, alice_faber_verkey,
                                                                transcript_cred_offer_json.encode('utf-8'))

    logger.info("\"Faber\" -> Send authcrypted \"Transcript\" Credential Offer to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Transcript\" Credential Offer from Faber")
    faber_alice_verkey, authdecrypted_transcript_cred_offer_json, authdecrypted_transcript_cred_offer = \
        await auth_decrypt(alice_wallet, alice_faber_key, authcrypted_transcript_cred_offer)

    logger.info("\"Alice\" -> Create and store \"Alice\" Master Secret in Wallet")
    alice_master_secret_id = await anoncreds.prover_create_master_secret(alice_wallet, None)

    logger.info("\"Alice\" -> Get \"Faber Transcript\" Credential Definition from Ledger")
    (faber_transcript_cred_def_id, faber_transcript_cred_def) = \
        await get_cred_def(pool_handle, alice_faber_did, authdecrypted_transcript_cred_offer['cred_def_id'])

    logger.info("\"Alice\" -> Create \"Transcript\" Credential Request for Faber")
    (transcript_cred_request_json, transcript_cred_request_metadata_json) = \
        await anoncreds.prover_create_credential_req(alice_wallet, alice_faber_did,
                                                     authdecrypted_transcript_cred_offer_json,
                                                     faber_transcript_cred_def, alice_master_secret_id)

    logger.info("\"Alice\" -> Authcrypt \"Transcript\" Credential Request for Faber")
    authcrypted_transcript_cred_request = await crypto.auth_crypt(alice_wallet, alice_faber_key, faber_alice_verkey,
                                                                  transcript_cred_request_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Transcript\" Credential Request to Faber")

    logger.info("\"Faber\" -> Authdecrypt \"Transcript\" Credential Request from Alice")
    alice_faber_verkey, authdecrypted_transcript_cred_request_json, _ = \
        await auth_decrypt(faber_wallet, faber_alice_key, authcrypted_transcript_cred_request)

    logger.info("\"Faber\" -> Create \"Transcript\" Credential for Alice")
    transcript_cred_values = json.dumps({
        "first_name": {"raw": "Alice", "encoded": "1139481716457488690172217916278103335"},
        "last_name": {"raw": "Garcia", "encoded": "5321642780241790123587902456789123452"},
        "degree": {"raw": "Bachelor of Science, Marketing", "encoded": "12434523576212321"},
        "status": {"raw": "graduated", "encoded": "2213454313412354"},
        "ssn": {"raw": "123-45-6789", "encoded": "3124141231422543541"},
        "year": {"raw": "2015", "encoded": "2015"},
        "average": {"raw": "5", "encoded": "5"}
    })

    transcript_cred_json, _, _ = \
        await anoncreds.issuer_create_credential(faber_wallet, transcript_cred_offer_json,
                                                 authdecrypted_transcript_cred_request_json,
                                                 transcript_cred_values, None, None)

    logger.info("\"Faber\" -> Authcrypt \"Transcript\" Credential for Alice")
    authcrypted_transcript_cred_json = await crypto.auth_crypt(faber_wallet, faber_alice_key, alice_faber_verkey,
                                                               transcript_cred_json.encode('utf-8'))

    logger.info("\"Faber\" -> Send authcrypted \"Transcript\" Credential to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Transcript\" Credential from Faber")
    _, authdecrypted_transcript_cred_json, _ = \
        await auth_decrypt(alice_wallet, alice_faber_key, authcrypted_transcript_cred_json)

    logger.info("\"Alice\" -> Store \"Transcript\" Credential from Faber")
    await anoncreds.prover_store_credential(alice_wallet, None, transcript_cred_request_json,
                                            transcript_cred_request_metadata_json, authdecrypted_transcript_cred_json,
                                            faber_transcript_cred_def, None)

    logger.info("==============================")
    logger.info("=== Apply for the job with Acme ==")
    logger.info("==============================")
    logger.info("== Apply for the job with Acme - Onboarding ==")
    logger.info("------------------------------")

    alice_wallet, alice_wallet_name, acme_alice_key, alice_acme_did, alice_acme_key, acme_alice_connection_response = \
        await onboarding(pool_handle, pool_name, "Acme", acme_wallet, acme_did, "Alice", alice_wallet, ' alice_wallet')

    logger.info("==============================")
    logger.info("== Apply for the job with Acme - Transcript proving ==")
    logger.info("------------------------------")

    logger.info("\"Acme\" -> Create \"Job-Application\" Proof Request")
    job_application_proof_request_json = json.dumps({
        'nonce': '1432422343242122312411212',
        'name': 'Job-Application',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {
                'name': 'first_name'
            },
            'attr2_referent': {
                'name': 'last_name'
            },
            'attr3_referent': {
                'name': 'degree',
                'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]
            },
            'attr4_referent': {
                'name': 'status',
                'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]
            },
            'attr5_referent': {
                'name': 'ssn',
                'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]
            },
            'attr6_referent': {
                'name': 'phone_number'
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'name': 'average',
                'p_type': '>=',
                'p_value': 4,
                'restrictions': [{'cred_def_id': faber_transcript_cred_def_id}]
            }
        }
    })

    logger.info("\"Acme\" -> Get key for Alice did")
    alice_acme_verkey = await did.key_for_did(pool_handle, acme_wallet, acme_alice_connection_response['did'])

    logger.info("\"Acme\" -> Authcrypt \"Job-Application\" Proof Request for Alice")
    authcrypted_job_application_proof_request_json = \
        await crypto.auth_crypt(acme_wallet, acme_alice_key, alice_acme_verkey,
                                job_application_proof_request_json.encode('utf-8'))

    logger.info("\"Acme\" -> Send authcrypted \"Job-Application\" Proof Request to Alice")

    logger.info("\"Alice\" -> Authdecrypt \"Job-Application\" Proof Request from Acme")
    acme_alice_verkey, authdecrypted_job_application_proof_request_json, _ = \
        await auth_decrypt(alice_wallet, alice_acme_key, authcrypted_job_application_proof_request_json)

    logger.info("\"Alice\" -> Get credentials for \"Job-Application\" Proof Request")
    creds_for_job_application_proof_request = json.loads(
        await anoncreds.prover_get_credentials_for_proof_req(alice_wallet,
                                                             authdecrypted_job_application_proof_request_json))
    print(creds_for_job_application_proof_request)

    cred_for_attr1 = creds_for_job_application_proof_request['attrs']['attr1_referent'][0]['cred_info']
    cred_for_attr2 = creds_for_job_application_proof_request['attrs']['attr2_referent'][0]['cred_info']
    cred_for_attr3 = creds_for_job_application_proof_request['attrs']['attr3_referent'][0]['cred_info']
    cred_for_attr4 = creds_for_job_application_proof_request['attrs']['attr4_referent'][0]['cred_info']
    cred_for_attr5 = creds_for_job_application_proof_request['attrs']['attr5_referent'][0]['cred_info']
    cred_for_predicate1 = creds_for_job_application_proof_request['predicates']['predicate1_referent'][0]['cred_info']

    creds_for_job_application_proof = {cred_for_attr1['referent']: cred_for_attr1,
                                       cred_for_attr2['referent']: cred_for_attr2,
                                       cred_for_attr3['referent']: cred_for_attr3,
                                       cred_for_attr4['referent']: cred_for_attr4,
                                       cred_for_attr5['referent']: cred_for_attr5,
                                       cred_for_predicate1['referent']: cred_for_predicate1}

    schemas_json, cred_defs_json, revoc_states_json = \
        await prover_get_entities_from_ledger(pool_handle, alice_faber_did, creds_for_job_application_proof, 'Alice')

    logger.info("\"Alice\" -> Create \"Job-Application\" Proof")
    job_application_requested_creds_json = json.dumps({
        'self_attested_attributes': {
            'attr1_referent': 'Alice',
            'attr2_referent': 'Garcia',
            'attr6_referent': '123-45-6789'
        },
        'requested_attributes': {
            'attr3_referent': {'cred_id': cred_for_attr3['referent'], 'revealed': True},
            'attr4_referent': {'cred_id': cred_for_attr4['referent'], 'revealed': True},
            'attr5_referent': {'cred_id': cred_for_attr5['referent'], 'revealed': True},
        },
        'requested_predicates': {'predicate1_referent': {'cred_id': cred_for_predicate1['referent']}}
    })

    job_application_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_job_application_proof_request_json,
                                            job_application_requested_creds_json, alice_master_secret_id,
                                            schemas_json, cred_defs_json, revoc_states_json)

    logger.info("\"Alice\" -> Authcrypt \"Job-Application\" Proof for Acme")
    authcrypted_job_application_proof_json = await crypto.auth_crypt(alice_wallet, alice_acme_key, acme_alice_verkey,
                                                                     job_application_proof_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Job-Application\" Proof to Acme")

    logger.info("\"Acme\" -> Authdecrypted \"Job-Application\" Proof from Alice")
    _, decrypted_job_application_proof_json, decrypted_job_application_proof = \
        await auth_decrypt(acme_wallet, acme_alice_key, authcrypted_job_application_proof_json)

    schemas_json, cred_defs_json, revoc_ref_defs_json, revoc_regs_json = \
        await verifier_get_entities_from_ledger(pool_handle, acme_did,
                                                decrypted_job_application_proof['identifiers'], 'Acme')

    logger.info("\"Acme\" -> Verify \"Job-Application\" Proof from Alice")
    assert 'Bachelor of Science, Marketing' == \
           decrypted_job_application_proof['requested_proof']['revealed_attrs']['attr3_referent']['raw']
    assert 'graduated' == \
           decrypted_job_application_proof['requested_proof']['revealed_attrs']['attr4_referent']['raw']
    assert '123-45-6789' == \
           decrypted_job_application_proof['requested_proof']['revealed_attrs']['attr5_referent']['raw']

    assert 'Alice' == decrypted_job_application_proof['requested_proof']['self_attested_attrs']['attr1_referent']
    assert 'Garcia' == decrypted_job_application_proof['requested_proof']['self_attested_attrs']['attr2_referent']
    assert '123-45-6789' == decrypted_job_application_proof['requested_proof']['self_attested_attrs']['attr6_referent']

    assert await anoncreds.verifier_verify_proof(job_application_proof_request_json,
                                                 decrypted_job_application_proof_json,
                                                 schemas_json, cred_defs_json, revoc_ref_defs_json, revoc_regs_json)

    logger.info("==============================")
    logger.info("== Apply for the job with Acme - Getting Job-Certificate Credential ==")
    logger.info("------------------------------")

    logger.info("\"Acme\" -> Create \"Job-Certificate\" Credential Offer for Alice")
    job_certificate_cred_offer_json = \
        await anoncreds.issuer_create_credential_offer(acme_wallet, acme_job_certificate_cred_def_id)

    logger.info("\"Acme\" -> Get key for Alice did")
    alice_acme_verkey = await did.key_for_did(pool_handle, acme_wallet, acme_alice_connection_response['did'])

    logger.info("\"Acme\" -> Authcrypt \"Job-Certificate\" Credential Offer for Alice")
    authcrypted_job_certificate_cred_offer = await crypto.auth_crypt(acme_wallet, acme_alice_key, alice_acme_verkey,
                                                                     job_certificate_cred_offer_json.encode('utf-8'))

    logger.info("\"Acme\" -> Send authcrypted \"Job-Certificate\" Credential Offer to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Job-Certificate\" Credential Offer from Acme")
    acme_alice_verkey, authdecrypted_job_certificate_cred_offer_json, authdecrypted_job_certificate_cred_offer = \
        await auth_decrypt(alice_wallet, alice_acme_key, authcrypted_job_certificate_cred_offer)

    logger.info("\"Alice\" -> Get \"Acme Job-Certificate\" Credential Definition from Ledger")
    (_, acme_job_certificate_cred_def) = \
        await get_cred_def(pool_handle, alice_acme_did, authdecrypted_job_certificate_cred_offer['cred_def_id'])

    logger.info("\"Alice\" -> Create and store in Wallet \"Job-Certificate\" Credential Request for Acme")
    (job_certificate_cred_request_json, job_certificate_cred_request_metadata_json) = \
        await anoncreds.prover_create_credential_req(alice_wallet, alice_acme_did,
                                                     authdecrypted_job_certificate_cred_offer_json,
                                                     acme_job_certificate_cred_def, alice_master_secret_id)

    logger.info("\"Alice\" -> Authcrypt \"Job-Certificate\" Credential Request for Acme")
    authcrypted_job_certificate_cred_request_json = \
        await crypto.auth_crypt(alice_wallet, alice_acme_key, acme_alice_verkey,
                                job_certificate_cred_request_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Job-Certificate\" Credential Request to Acme")

    logger.info("\"Acme\" -> Authdecrypt \"Job-Certificate\" Credential Request from Alice")
    alice_acme_verkey, authdecrypted_job_certificate_cred_request_json, _ = \
        await auth_decrypt(acme_wallet, acme_alice_key, authcrypted_job_certificate_cred_request_json)

    logger.info("\"Acme\" -> Create \"Job-Certificate\" Credential for Alice")
    alice_job_certificate_cred_values_json = json.dumps({
        "first_name": {"raw": "Alice", "encoded": "245712572474217942457235975012103335"},
        "last_name": {"raw": "Garcia", "encoded": "312643218496194691632153761283356127"},
        "employee_status": {"raw": "Permanent", "encoded": "2143135425425143112321314321"},
        "salary": {"raw": "2400", "encoded": "2400"},
        "experience": {"raw": "10", "encoded": "10"}
    })

    job_certificate_cred_json, _, _ = \
        await anoncreds.issuer_create_credential(acme_wallet, job_certificate_cred_offer_json,
                                                 authdecrypted_job_certificate_cred_request_json,
                                                 alice_job_certificate_cred_values_json, None, None)

    logger.info("\"Acme\" ->  Authcrypt \"Job-Certificate\" Credential for Alice")
    authcrypted_job_certificate_cred_json = \
        await crypto.auth_crypt(acme_wallet, acme_alice_key, alice_acme_verkey,
                                job_certificate_cred_json.encode('utf-8'))

    logger.info("\"Acme\" ->  Send authcrypted \"Job-Certificate\" Credential to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Job-Certificate\" Credential from Acme")
    _, authdecrypted_job_certificate_cred_json, _ = \
        await auth_decrypt(alice_wallet, alice_acme_key, authcrypted_job_certificate_cred_json)

    logger.info("\"Alice\" -> Store \"Job-Certificate\" Credential")
    await anoncreds.prover_store_credential(alice_wallet, None, job_certificate_cred_request_json,
                                            job_certificate_cred_request_metadata_json,
                                            authdecrypted_job_certificate_cred_json,
                                            acme_job_certificate_cred_def_json, None)

    logger.info("==============================")
    logger.info("=== Apply for the loan with Thrift ==")
    logger.info("==============================")
    logger.info("== Apply for the loan with Thrift - Onboarding ==")
    logger.info("------------------------------")

    alice_wallet, alice_wallet_name, thrift_alice_key, alice_thrift_did, alice_thrift_key, \
    thrift_alice_connection_response = await onboarding(pool_handle, pool_name, "Thrift", thrift_wallet, thrift_did,
                                                        "Alice", alice_wallet, ' alice_wallet')

    logger.info("==============================")
    logger.info("== Apply for the loan with Thrift - Job-Certificate proving  ==")
    logger.info("------------------------------")

    logger.info("\"Thrift\" -> Create \"Loan-Application-Basic\" Proof Request")
    apply_loan_proof_request_json = json.dumps({
        'nonce': '123432421212',
        'name': 'Loan-Application-Basic',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {
                'name': 'employee_status',
                'restrictions': [{'cred_def_id': acme_job_certificate_cred_def_id}]
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'name': 'salary',
                'p_type': '>=',
                'p_value': 2000,
                'restrictions': [{'cred_def_id': acme_job_certificate_cred_def_id}]
            },
            'predicate2_referent': {
                'name': 'experience',
                'p_type': '>=',
                'p_value': 1,
                'restrictions': [{'cred_def_id': acme_job_certificate_cred_def_id}]
            }
        }
    })

    logger.info("\"Thrift\" -> Get key for Alice did")
    alice_thrift_verkey = await did.key_for_did(pool_handle, thrift_wallet, thrift_alice_connection_response['did'])

    logger.info("\"Thrift\" -> Authcrypt \"Loan-Application-Basic\" Proof Request for Alice")
    authcrypted_apply_loan_proof_request_json = \
        await crypto.auth_crypt(thrift_wallet, thrift_alice_key, alice_thrift_verkey,
                                apply_loan_proof_request_json.encode('utf-8'))

    logger.info("\"Thrift\" -> Send authcrypted \"Loan-Application-Basic\" Proof Request to Alice")

    logger.info("\"Alice\" -> Authdecrypt \"Loan-Application-Basic\" Proof Request from Thrift")
    thrift_alice_verkey, authdecrypted_apply_loan_proof_request_json, _ = \
        await auth_decrypt(alice_wallet, alice_thrift_key, authcrypted_apply_loan_proof_request_json)

    logger.info("\"Alice\" -> Get credentials for \"Loan-Application-Basic\" Proof Request")
    creds_json_for_apply_loan_proof_request = \
        await anoncreds.prover_get_credentials_for_proof_req(alice_wallet, authdecrypted_apply_loan_proof_request_json)
    creds_for_apply_loan_proof_request = json.loads(creds_json_for_apply_loan_proof_request)

    cred_for_attr1 = creds_for_apply_loan_proof_request['attrs']['attr1_referent'][0]['cred_info']
    cred_for_predicate1 = creds_for_apply_loan_proof_request['predicates']['predicate1_referent'][0]['cred_info']
    cred_for_predicate2 = creds_for_apply_loan_proof_request['predicates']['predicate2_referent'][0]['cred_info']

    creds_for_apply_loan_proof = {cred_for_attr1['referent']: cred_for_attr1,
                                  cred_for_predicate1['referent']: cred_for_predicate1,
                                  cred_for_predicate2['referent']: cred_for_predicate2}

    schemas_json, cred_defs_json, revoc_states_json = \
        await prover_get_entities_from_ledger(pool_handle, alice_thrift_did, creds_for_apply_loan_proof, 'Alice')

    logger.info("\"Alice\" -> Create \"Loan-Application-Basic\" Proof")
    apply_loan_requested_creds_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {
            'attr1_referent': {'cred_id': cred_for_attr1['referent'], 'revealed': True}
        },
        'requested_predicates': {
            'predicate1_referent': {'cred_id': cred_for_predicate1['referent']},
            'predicate2_referent': {'cred_id': cred_for_predicate2['referent']}
        }
    })
    alice_apply_loan_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_apply_loan_proof_request_json,
                                            apply_loan_requested_creds_json, alice_master_secret_id, schemas_json,
                                            cred_defs_json, revoc_states_json)

    logger.info("\"Alice\" -> Authcrypt \"Loan-Application-Basic\" Proof for Thrift")
    authcrypted_alice_apply_loan_proof_json = \
        await crypto.auth_crypt(alice_wallet, alice_thrift_key, thrift_alice_verkey,
                                alice_apply_loan_proof_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Loan-Application-Basic\" Proof to Thrift")

    logger.info("\"Thrift\" -> Authdecrypted \"Loan-Application-Basic\" Proof from Alice")
    _, authdecrypted_alice_apply_loan_proof_json, authdecrypted_alice_apply_loan_proof = \
        await auth_decrypt(thrift_wallet, thrift_alice_key, authcrypted_alice_apply_loan_proof_json)

    logger.info("\"Thrift\" -> Get Schemas, Credential Definitions and Revocation Registries from Ledger"
                " required for Proof verifying")

    schemas_json, cred_defs_json, revoc_defs_json, revoc_regs_json = \
        await verifier_get_entities_from_ledger(pool_handle, thrift_did,
                                                authdecrypted_alice_apply_loan_proof['identifiers'], 'Thrift')

    logger.info("\"Thrift\" -> Verify \"Loan-Application-Basic\" Proof from Alice")
    assert 'Permanent' == \
           authdecrypted_alice_apply_loan_proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']

    assert await anoncreds.verifier_verify_proof(apply_loan_proof_request_json,
                                                 authdecrypted_alice_apply_loan_proof_json,
                                                 schemas_json, cred_defs_json, revoc_defs_json, revoc_regs_json)

    logger.info("==============================")

    logger.info("==============================")
    logger.info("== Apply for the loan with Thrift - Transcript and Job-Certificate proving  ==")
    logger.info("------------------------------")

    logger.info("\"Thrift\" -> Create \"Loan-Application-KYC\" Proof Request")
    apply_loan_kyc_proof_request_json = json.dumps({
        'nonce': '123432421212',
        'name': 'Loan-Application-KYC',
        'version': '0.1',
        'requested_attributes': {
            'attr1_referent': {'name': 'first_name'},
            'attr2_referent': {'name': 'last_name'},
            'attr3_referent': {'name': 'ssn'}
        },
        'requested_predicates': {}
    })

    logger.info("\"Thrift\" -> Get key for Alice did")
    alice_thrift_verkey = await did.key_for_did(pool_handle, thrift_wallet, thrift_alice_connection_response['did'])

    logger.info("\"Thrift\" -> Authcrypt \"Loan-Application-KYC\" Proof Request for Alice")
    authcrypted_apply_loan_kyc_proof_request_json = \
        await crypto.auth_crypt(thrift_wallet, thrift_alice_key, alice_thrift_verkey,
                                apply_loan_kyc_proof_request_json.encode('utf-8'))

    logger.info("\"Thrift\" -> Send authcrypted \"Loan-Application-KYC\" Proof Request to Alice")

    logger.info("\"Alice\" -> Authdecrypt \"Loan-Application-KYC\" Proof Request from Thrift")
    thrift_alice_verkey, authdecrypted_apply_loan_kyc_proof_request_json, _ = \
        await auth_decrypt(alice_wallet, alice_thrift_key, authcrypted_apply_loan_kyc_proof_request_json)

    logger.info("\"Alice\" -> Get credentials for \"Loan-Application-KYC\" Proof Request")
    creds_json_for_apply_loan_kyc_proof_request = \
        await anoncreds.prover_get_credentials_for_proof_req(alice_wallet,
                                                             authdecrypted_apply_loan_kyc_proof_request_json)
    creds_for_apply_loan_kyc_proof_request = json.loads(creds_json_for_apply_loan_kyc_proof_request)

    cred_for_attr1 = creds_for_apply_loan_kyc_proof_request['attrs']['attr1_referent'][0]['cred_info']
    cred_for_attr2 = creds_for_apply_loan_kyc_proof_request['attrs']['attr2_referent'][0]['cred_info']
    cred_for_attr3 = creds_for_apply_loan_kyc_proof_request['attrs']['attr3_referent'][0]['cred_info']

    creds_for_apply_loan_kyc_proof = {cred_for_attr1['referent']: cred_for_attr1,
                                      cred_for_attr2['referent']: cred_for_attr2,
                                      cred_for_attr3['referent']: cred_for_attr3}

    schemas_json, cred_defs_json, revoc_states_json = \
        await prover_get_entities_from_ledger(pool_handle, alice_thrift_did, creds_for_apply_loan_kyc_proof, 'Alice')

    logger.info("\"Alice\" -> Create \"Loan-Application-KYC\" Proof")

    apply_loan_kyc_requested_creds_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attributes': {
            'attr1_referent': {'cred_id': cred_for_attr1['referent'], 'revealed': True},
            'attr2_referent': {'cred_id': cred_for_attr2['referent'], 'revealed': True},
            'attr3_referent': {'cred_id': cred_for_attr3['referent'], 'revealed': True}
        },
        'requested_predicates': {}
    })

    alice_apply_loan_kyc_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_apply_loan_kyc_proof_request_json,
                                            apply_loan_kyc_requested_creds_json, alice_master_secret_id,
                                            schemas_json, cred_defs_json, revoc_states_json)

    logger.info("\"Alice\" -> Authcrypt \"Loan-Application-KYC\" Proof for Thrift")
    authcrypted_alice_apply_loan_kyc_proof_json = \
        await crypto.auth_crypt(alice_wallet, alice_thrift_key, thrift_alice_verkey,
                                alice_apply_loan_kyc_proof_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Loan-Application-KYC\" Proof to Thrift")

    logger.info("\"Thrift\" -> Authdecrypted \"Loan-Application-KYC\" Proof from Alice")
    _, authdecrypted_alice_apply_loan_kyc_proof_json, authdecrypted_alice_apply_loan_kyc_proof = \
        await auth_decrypt(thrift_wallet, thrift_alice_key, authcrypted_alice_apply_loan_kyc_proof_json)

    logger.info("\"Thrift\" -> Get Schemas, Credential Definitions and Revocation Registries from Ledger"
                " required for Proof verifying")

    schemas_json, cred_defs_json, revoc_defs_json, revoc_regs_json = \
        await verifier_get_entities_from_ledger(pool_handle, thrift_did,
                                                authdecrypted_alice_apply_loan_kyc_proof['identifiers'], 'Thrift')

    logger.info("\"Thrift\" -> Verify \"Loan-Application-KYC\" Proof from Alice")
    assert 'Alice' == \
           authdecrypted_alice_apply_loan_kyc_proof['requested_proof']['revealed_attrs']['attr1_referent']['raw']
    assert 'Garcia' == \
           authdecrypted_alice_apply_loan_kyc_proof['requested_proof']['revealed_attrs']['attr2_referent']['raw']
    assert '123-45-6789' == \
           authdecrypted_alice_apply_loan_kyc_proof['requested_proof']['revealed_attrs']['attr3_referent']['raw']

    assert await anoncreds.verifier_verify_proof(apply_loan_kyc_proof_request_json,
                                                 authdecrypted_alice_apply_loan_kyc_proof_json,
                                                 schemas_json, cred_defs_json, revoc_defs_json, revoc_regs_json)

    logger.info("==============================")

    logger.info(" \"Sovrin Steward\" -> Close and Delete wallet")
    await wallet.close_wallet(steward_wallet)
    await wallet.delete_wallet(steward_wallet_name, None)

    logger.info("\"Government\" -> Close and Delete wallet")
    await wallet.close_wallet(government_wallet)
    await wallet.delete_wallet(government_wallet_name, None)

    logger.info("\"Faber\" -> Close and Delete wallet")
    await wallet.close_wallet(faber_wallet)
    await wallet.delete_wallet(faber_wallet_name, None)

    logger.info("\"Acme\" -> Close and Delete wallet")
    await wallet.close_wallet(acme_wallet)
    await wallet.delete_wallet(acme_wallet_name, None)

    logger.info("\"Thrift\" -> Close and Delete wallet")
    await wallet.close_wallet(thrift_wallet)
    await wallet.delete_wallet(thrift_wallet_name, None)

    logger.info("\"Alice\" -> Close and Delete wallet")
    await wallet.close_wallet(alice_wallet)
    await wallet.delete_wallet(alice_wallet_name, None)

    logger.info("Close and Delete pool")
    await pool.close_pool_ledger(pool_handle)
    await pool.delete_pool_ledger_config(pool_name)

    logger.info("Getting started -> done")


async def onboarding(pool_handle, pool_name, _from, from_wallet, from_did, to,
                     to_wallet: Optional[str],
                     to_wallet_name: Optional[str]):
    logger.info("\"{}\" -> Create and store in Wallet \"{} {}\" DID".format(_from, _from, to))
    (from_to_did, from_to_key) = await did.create_and_store_my_did(from_wallet, "{}")

    logger.info("\"{}\" -> Send Nym to Ledger for \"{} {}\" DID".format(_from, _from, to))
    await send_nym(pool_handle, from_wallet, from_did, from_to_did, from_to_key, None)

    logger.info("\"{}\" -> Send connection request to {} with \"{} {}\" DID and nonce".format(_from, to, _from, to))
    connection_request = {
        'did': from_to_did,
        'nonce': 123456789
    }

    if not to_wallet:
        logger.info("\"{}\" -> Create wallet".format(to))
        try:
            await wallet.create_wallet(pool_name, to_wallet_name, None, None,
                                       None)
        except IndyError as ex:
            if ex.error_code == ErrorCode.PoolLedgerConfigAlreadyExistsError:
                pass
        to_wallet = await wallet.open_wallet(to_wallet_name, None, None)

    logger.info("\"{}\" -> Create and store in Wallet \"{} {}\" DID".format(to, to, _from))
    (to_from_did, to_from_key) = await did.create_and_store_my_did(to_wallet, "{}")

    logger.info("\"{}\" -> Get key for did from \"{}\" connection request".format(to, _from))
    from_to_verkey = await did.key_for_did(pool_handle, to_wallet, connection_request['did'])

    logger.info("\"{}\" -> Anoncrypt connection response for \"{}\" with \"{} {}\" DID, verkey and nonce"
                .format(to, _from, to, _from))
    connection_response = json.dumps({
        'did': to_from_did,
        'verkey': to_from_key,
        'nonce': connection_request['nonce']
    })
    anoncrypted_connection_response = await crypto.anon_crypt(from_to_verkey, connection_response.encode('utf-8'))

    logger.info("\"{}\" -> Send anoncrypted connection response to \"{}\"".format(to, _from))

    logger.info("\"{}\" -> Anondecrypt connection response from \"{}\"".format(_from, to))
    decrypted_connection_response = \
        json.loads((await crypto.anon_decrypt(from_wallet, from_to_key,
                                              anoncrypted_connection_response)).decode("utf-8"))

    logger.info("\"{}\" -> Authenticates \"{}\" by comparision of Nonce".format(_from, to))
    assert connection_request['nonce'] == decrypted_connection_response['nonce']

    logger.info("\"{}\" -> Send Nym to Ledger for \"{} {}\" DID".format(_from, to, _from))
    await send_nym(pool_handle, from_wallet, from_did, to_from_did, to_from_key, None)

    return to_wallet, to_wallet_name, from_to_key, to_from_did, to_from_key, decrypted_connection_response


async def get_verinym(pool_handle, _from, from_wallet, from_did, from_to_key,
                      to, to_wallet, to_from_did, to_from_key, role):
    logger.info("\"{}\" -> Create and store in Wallet \"{}\" new DID".format(to, to))
    (to_did, to_key) = await did.create_and_store_my_did(to_wallet, "{}")

    logger.info("\"{}\" -> Authcrypt \"{} DID info\" for \"{}\"".format(to, to, _from))
    did_info_json = json.dumps({
        'did': to_did,
        'verkey': to_key
    })
    authcrypted_did_info_json = \
        await crypto.auth_crypt(to_wallet, to_from_key, from_to_key, did_info_json.encode('utf-8'))

    logger.info("\"{}\" -> Send authcrypted \"{} DID info\" to {}".format(to, to, _from))

    logger.info("\"{}\" -> Authdecrypted \"{} DID info\" from {}".format(_from, to, to))
    sender_verkey, authdecrypted_did_info_json, authdecrypted_did_info = \
        await auth_decrypt(from_wallet, from_to_key, authcrypted_did_info_json)

    logger.info("\"{}\" -> Authenticate {} by comparision of Verkeys".format(_from, to, ))
    assert sender_verkey == await did.key_for_did(pool_handle, from_wallet, to_from_did)

    logger.info("\"{}\" -> Send Nym to Ledger for \"{} DID\" with {} Role".format(_from, to, role))
    await send_nym(pool_handle, from_wallet, from_did, authdecrypted_did_info['did'],
                   authdecrypted_did_info['verkey'], role)

    return to_did


async def send_nym(pool_handle, wallet_handle, _did, new_did, new_key, role):
    nym_request = await ledger.build_nym_request(_did, new_did, new_key, None, role)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, _did, nym_request)


async def send_schema(pool_handle, wallet_handle, _did, schema):
    schema_request = await ledger.build_schema_request(_did, schema)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, _did, schema_request)


async def send_cred_def(pool_handle, wallet_handle, _did, cred_def_json):
    cred_def_request = await ledger.build_cred_def_request(_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, _did, cred_def_request)


async def get_schema(pool_handle, _did, schema_id):
    get_schema_request = await ledger.build_get_schema_request(_did, schema_id)
    get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
    return await ledger.parse_get_schema_response(get_schema_response)


async def get_cred_def(pool_handle, _did, schema_id):
    get_cred_def_request = await ledger.build_get_cred_def_request(_did, schema_id)
    get_cred_def_response = await ledger.submit_request(pool_handle, get_cred_def_request)
    return await ledger.parse_get_cred_def_response(get_cred_def_response)


async def prover_get_entities_from_ledger(pool_handle, _did, identifiers, actor):
    schemas = {}
    cred_defs = {}
    rev_states = {}
    for item in identifiers.values():
        logger.info("\"{}\" -> Get Schema from Ledger".format(actor))
        (received_schema_id, received_schema) = await get_schema(pool_handle, _did, item['schema_id'])
        schemas[received_schema_id] = json.loads(received_schema)

        logger.info("\"{}\" -> Get Claim Definition from Ledger".format(actor))
        (received_cred_def_id, received_cred_def) = await get_cred_def(pool_handle, _did, item['cred_def_id'])
        cred_defs[received_cred_def_id] = json.loads(received_cred_def)

        if 'rev_reg_seq_no' in item:
            pass  # TODO Create Revocation States

    return json.dumps(schemas), json.dumps(cred_defs), json.dumps(rev_states)


async def verifier_get_entities_from_ledger(pool_handle, _did, identifiers, actor):
    schemas = {}
    cred_defs = {}
    rev_reg_defs = {}
    rev_regs = {}
    for item in identifiers:
        logger.info("\"{}\" -> Get Schema from Ledger".format(actor))
        (received_schema_id, received_schema) = await get_schema(pool_handle, _did, item['schema_id'])
        schemas[received_schema_id] = json.loads(received_schema)

        logger.info("\"{}\" -> Get Claim Definition from Ledger".format(actor))
        (received_cred_def_id, received_cred_def) = await get_cred_def(pool_handle, _did, item['cred_def_id'])
        cred_defs[received_cred_def_id] = json.loads(received_cred_def)

        if 'rev_reg_seq_no' in item:
            pass  # TODO Get Revocation Definitions and Revocation Registries

    return json.dumps(schemas), json.dumps(cred_defs), json.dumps(rev_reg_defs), json.dumps(rev_regs)


async def auth_decrypt(wallet_handle, key, message):
    from_verkey, decrypted_message_json = await crypto.auth_decrypt(wallet_handle, key, message)
    decrypted_message_json = decrypted_message_json.decode("utf-8")
    decrypted_message = json.loads(decrypted_message_json)
    return from_verkey, decrypted_message_json, decrypted_message


if __name__ == '__main__':
    run_coroutine(run)
    time.sleep(1)  # FIXME waiting for libindy thread complete
