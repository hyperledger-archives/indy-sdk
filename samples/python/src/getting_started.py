from indy import anoncreds, crypto, did, ledger, pool, wallet

import json
import logging
from typing import Optional

from src.utils import get_pool_genesis_txn_path

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


async def run():
    logger.info("Getting started -> started")

    logger.info("Open Pool Ledger")
    pool_name = 'pool1'
    pool_genesis_txn_path = get_pool_genesis_txn_path(pool_name)
    pool_config = json.dumps({"genesis_txn": str(pool_genesis_txn_path)})
    await pool.create_pool_ledger_config(pool_name, pool_config)
    pool_handle = await pool.open_pool_ledger(pool_name, None)

    logger.info("==============================")
    logger.info("=== Getting Trust Anchor credentials for Faber, Acme, Thrift and Government  ==")
    logger.info("------------------------------")

    logger.info("\"Sovrin Steward\" -> Create wallet")
    steward_wallet_name = 'sovrin_steward_wallet'
    await wallet.create_wallet(pool_name, steward_wallet_name, None, None, None)
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
    logger.info("=== Claim Schemas Setup ==")
    logger.info("------------------------------")

    logger.info("\"Government\" -> Create and store in Wallet \"Government Issuer\" DID")
    (government_issuer_did, government_issuer_key) = await did.create_and_store_my_did(government_wallet, "{}")

    logger.info("\"Government\" -> Send Nym to Ledger for \"Government Issuer\" DID")
    await send_nym(pool_handle, government_wallet, government_did, government_issuer_did, government_issuer_key, None)

    logger.info("\"Government\" -> Send to Ledger \"Job-Certificate\" Schema")
    job_certificate_schema = {
        'name': 'Job-Certificate',
        'version': '0.2',
        'attr_names': ['first_name', 'last_name', 'salary', 'employee_status', 'experience']
    }
    job_certificate_schema_key = {
        'name': job_certificate_schema['name'],
        'version': job_certificate_schema['version'],
        'did': government_issuer_did
    }
    await send_schema(pool_handle, government_wallet, government_issuer_did, job_certificate_schema)

    logger.info("\"Government\" -> Send to Ledger \"Transcript\" Schema")
    transcript_schema = {
        'name': 'Transcript',
        'version': '1.2',
        'attr_names': ['first_name', 'last_name', 'degree', 'status', 'year', 'average', 'ssn']
    }
    transcript_schema_key = {
        'name': transcript_schema['name'],
        'version': transcript_schema['version'],
        'did': government_issuer_did
    }
    await send_schema(pool_handle, government_wallet, government_issuer_did, transcript_schema)

    logger.info("==============================")
    logger.info("=== Faber Claim Definition Setup ==")
    logger.info("------------------------------")

    logger.info("\"Faber\" -> Create and store in Wallet \"Faber Issuer\" DID")
    (faber_issuer_did, faber_issuer_key) = await did.create_and_store_my_did(faber_wallet, "{}")

    logger.info("\"Faber\" -> Send Nym to Ledger for \"Faber Issuer\" DID")
    await send_nym(pool_handle, faber_wallet, faber_did, faber_issuer_did, faber_issuer_key, None)

    logger.info("\"Faber\" -> Get \"Transcript\" Schema from Ledger")
    transcript_schema = await get_schema(pool_handle, faber_issuer_did, transcript_schema_key)

    logger.info("\"Faber\" -> Create and store in Wallet \"Faber Transcript\" Claim Definition")
    faber_transcript_claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(faber_wallet, faber_issuer_did,
                                                          json.dumps(transcript_schema), 'CL', False)

    logger.info("\"Faber\" -> Send ClaimDef to Ledger for \"Faber Transcript\" Claim Definition")
    await send_claim_def(pool_handle, faber_wallet, faber_issuer_did, faber_transcript_claim_def_json)

    logger.info("==============================")
    logger.info("=== Acme Claim Definition Setup ==")
    logger.info("------------------------------")

    logger.info("\"Acme\" -> Create and store in Wallet \"Acme Issuer\" DID")
    (acme_issuer_did, acme_issuer_key) = await did.create_and_store_my_did(acme_wallet, "{}")

    logger.info("\"Acme\" -> Send Nym to Ledger for \"Acme Issuer\" DID")
    await send_nym(pool_handle, acme_wallet, acme_did, acme_issuer_did, acme_issuer_key, None)

    logger.info("\"Acme\" ->  Get from Ledger \"Job-Certificate\" Schema")
    job_certificate_schema = await get_schema(pool_handle, acme_issuer_did, job_certificate_schema_key)

    logger.info("\"Acme\" -> Create and store in Wallet \"Acme Job-Certificate\" Claim Definition")
    acme_job_certificate_claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(acme_wallet, acme_issuer_did,
                                                          json.dumps(job_certificate_schema), 'CL', False)

    logger.info("\"Acme\" -> Send ClaimDef to Ledger for \"Acme Job-Certificate\" Claim Definition")
    await send_claim_def(pool_handle, acme_wallet, acme_issuer_did, acme_job_certificate_claim_def_json)

    logger.info("==============================")
    logger.info("=== Getting Transcript with Faber ==")
    logger.info("==============================")
    logger.info("== Getting Transcript with Faber - Onboarding ==")
    logger.info("------------------------------")

    alice_wallet, alice_wallet_name, faber_alice_key, alice_faber_did, alice_faber_key, faber_alice_connection_response \
        = await onboarding(pool_handle, pool_name, "Faber", faber_wallet, faber_did, "Alice", None, ' alice_wallet')

    logger.info("==============================")
    logger.info("== Getting Transcript with Faber - Getting Transcript Claim ==")
    logger.info("------------------------------")

    logger.info("\"Faber\" -> Create \"Transcript\" Claim Offer for Alice")
    transcript_claim_offer_json = \
        await anoncreds.issuer_create_claim_offer(faber_wallet, json.dumps(transcript_schema),
                                                  faber_issuer_did, alice_faber_did)

    logger.info("\"Faber\" -> Get key for Alice did")
    alice_faber_verkey = await did.key_for_did(pool_handle, acme_wallet, faber_alice_connection_response['did'])

    logger.info("\"Faber\" -> Authcrypt \"Transcript\" Claim Offer for Alice")
    authcrypted_transcript_claim_offer = await crypto.auth_crypt(faber_wallet, faber_alice_key, alice_faber_verkey,
                                                                 transcript_claim_offer_json)

    logger.info("\"Faber\" -> Send authcrypted \"Transcript\" Claim Offer to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Transcript\" Claim Offer from Faber")
    faber_alice_verkey, authdecrypted_transcript_claim_offer_json, authdecrypted_transcript_claim_offer = \
        await auth_decrypt(alice_wallet, alice_faber_key, authcrypted_transcript_claim_offer)

    logger.info("\"Alice\" -> Store \"Transcript\" Claim Offer in Wallet from Faber")
    await anoncreds.prover_store_claim_offer(alice_wallet, authdecrypted_transcript_claim_offer_json)

    logger.info("\"Alice\" -> Create and store \"Alice\" Master Secret in Wallet")
    alice_master_secret_name = 'alice_master_secret'
    await anoncreds.prover_create_master_secret(alice_wallet, alice_master_secret_name)

    logger.info("\"Alice\" -> Get \"Transcript\" Schema from Ledger")
    transcript_schema = await get_schema(pool_handle, alice_faber_did,
                                         authdecrypted_transcript_claim_offer['schema_key'])

    logger.info("\"Alice\" -> Get \"Faber Transcript\" Claim Definition from Ledger")
    faber_transcript_claim_def = await get_claim_def(pool_handle, alice_faber_did, transcript_schema,
                                                     authdecrypted_transcript_claim_offer['issuer_did'])

    logger.info("\"Alice\" -> Create and store in Wallet \"Transcript\" Claim Request for Faber")
    transcript_claim_request_json = \
        await anoncreds.prover_create_and_store_claim_req(alice_wallet, alice_faber_did,
                                                          authdecrypted_transcript_claim_offer_json,
                                                          json.dumps(faber_transcript_claim_def),
                                                          alice_master_secret_name)

    logger.info("\"Alice\" -> Authcrypt \"Transcript\" Claim Request for Faber")
    authcrypted_transcript_claim_request = await crypto.auth_crypt(alice_wallet, alice_faber_key, faber_alice_verkey,
                                                                   transcript_claim_request_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Transcript\" Claim Request to Faber")

    logger.info("\"Faber\" -> Authdecrypt \"Transcript\" Claim Request from Alice")
    alice_faber_verkey, authdecrypted_transcript_claim_request_json, _ = \
        await auth_decrypt(faber_wallet, faber_alice_key, authcrypted_transcript_claim_request)

    logger.info("\"Faber\" -> Create \"Transcript\" Claim for Alice")
    transcript_claim_values = json.dumps({
        'first_name': ['Alice', '1139481716457488690172217916278103335'],
        'last_name': ['Garcia', '5321642780241790123587902456789123452'],
        'degree': ['Bachelor of Science, Marketing', '12434523576212321'],
        'status': ['graduated', '2213454313412354'],
        'ssn': ['123-45-6789', '3124141231422543541'],
        'year': ['2015', '2015'],
        'average': ['5', '5']
    })

    _, transcript_claim_json = \
        await anoncreds.issuer_create_claim(faber_wallet, authdecrypted_transcript_claim_request_json,
                                            transcript_claim_values, -1)

    logger.info("\"Faber\" -> Authcrypt \"Transcript\" Claim for Alice")
    authcrypted_transcript_claim_json = await crypto.auth_crypt(faber_wallet, faber_alice_key, alice_faber_verkey,
                                                                transcript_claim_json.encode('utf-8'))

    logger.info("\"Faber\" -> Send authcrypted \"Transcript\" Claim to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Transcript\" Claim from Faber")
    _, authdecrypted_transcript_claim_json, _ = \
        await auth_decrypt(alice_wallet, alice_faber_key, authcrypted_transcript_claim_json)

    logger.info("\"Alice\" -> Store \"Transcript\" Claim from Faber")
    await anoncreds.prover_store_claim(alice_wallet, authdecrypted_transcript_claim_json, None)

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
        'requested_attrs': {
            'attr1_referent': {
                'name': 'first_name'
            },
            'attr2_referent': {
                'name': 'last_name'
            },
            'attr3_referent': {
                'name': 'degree',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
            },
            'attr4_referent': {
                'name': 'status',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
            },
            'attr5_referent': {
                'name': 'ssn',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
            },
            'attr6_referent': {
                'name': 'phone_number'
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'attr_name': 'average',
                'p_type': '>=',
                'value': 4,
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': transcript_schema_key}]
            }
        }
    })

    logger.info("\"Acme\" -> Get key for Alice did")
    alice_acme_verkey = await did.key_for_did(pool_handle, acme_wallet, acme_alice_connection_response['did'])

    logger.info("\"Acme\" -> Authcrypt \"Job-Application\" Proof Request for Alice")
    authcrypted_job_application_proof_request_json = \
        await crypto.auth_crypt(acme_wallet, acme_alice_key, alice_acme_verkey,
                                job_application_proof_request_json.encode('utf-8'))

    logger.info("\"Acme\" -> Sentd authcrypted \"Job-Application\" Proof Request to Alice")

    logger.info("\"Alice\" -> Authdecrypt \"Job-Application\" Proof Request from Acme")
    acme_alice_verkey, authdecrypted_job_application_proof_request_json, _ = \
        await auth_decrypt(alice_wallet, alice_acme_key, authcrypted_job_application_proof_request_json)

    logger.info("\"Alice\" -> Get claims for \"Job-Application\" Proof Request")
    claims_for_job_application_proof_request = json.loads(
        await anoncreds.prover_get_claims_for_proof_req(alice_wallet, authdecrypted_job_application_proof_request_json))

    claim_for_attr1 = claims_for_job_application_proof_request['attrs']['attr1_referent'][0]
    claim_for_attr2 = claims_for_job_application_proof_request['attrs']['attr2_referent'][0]
    claim_for_attr3 = claims_for_job_application_proof_request['attrs']['attr3_referent'][0]
    claim_for_attr4 = claims_for_job_application_proof_request['attrs']['attr4_referent'][0]
    claim_for_attr5 = claims_for_job_application_proof_request['attrs']['attr5_referent'][0]
    claim_for_predicate1 = claims_for_job_application_proof_request['predicates']['predicate1_referent'][0]

    claims_for_job_application_proof = {claim_for_attr1['referent']: claim_for_attr1,
                                        claim_for_attr2['referent']: claim_for_attr2,
                                        claim_for_attr3['referent']: claim_for_attr3,
                                        claim_for_attr4['referent']: claim_for_attr4,
                                        claim_for_attr5['referent']: claim_for_attr5,
                                        claim_for_predicate1['referent']: claim_for_predicate1}

    schemas_json, claim_defs_json, revoc_regs_json = \
        await get_entities_from_ledger(pool_handle, alice_faber_did, claims_for_job_application_proof, 'Alice')

    logger.info("\"Alice\" -> Create \"Job-Application\" Proof")
    job_application_requested_claims_json = json.dumps({
        'self_attested_attributes': {
            'attr1_referent': 'Alice',
            'attr2_referent': 'Garcia',
            'attr6_referent': '123-45-6789'
        },
        'requested_attrs': {
            'attr3_referent': [claim_for_attr3['referent'], True],
            'attr4_referent': [claim_for_attr4['referent'], True],
            'attr5_referent': [claim_for_attr5['referent'], True],
        },
        'requested_predicates': {'predicate1_referent': claim_for_predicate1['referent']}
    })

    job_application_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_job_application_proof_request_json,
                                            job_application_requested_claims_json, schemas_json,
                                            alice_master_secret_name, claim_defs_json, revoc_regs_json)

    logger.info("\"Alice\" -> Authcrypt \"Job-Application\" Proof for Acme")
    authcrypted_job_application_proof_json = await crypto.auth_crypt(alice_wallet, alice_acme_key, acme_alice_verkey,
                                                                     job_application_proof_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Job-Application\" Proof to Acme")

    logger.info("\"Acme\" -> Authdecrypted \"Job-Application\" Proof from Alice")
    _, decrypted_job_application_proof_json, decrypted_job_application_proof = \
        await auth_decrypt(acme_wallet, acme_alice_key, authcrypted_job_application_proof_json)

    schemas_json, claim_defs_json, revoc_regs_json = \
        await get_entities_from_ledger(pool_handle, acme_did, decrypted_job_application_proof['identifiers'], 'Acme')

    logger.info("\"Acme\" -> Verify \"Job-Application\" Proof from Alice")
    assert 'Bachelor of Science, Marketing' == \
           decrypted_job_application_proof['requested_proof']['revealed_attrs']['attr3_referent'][1]
    assert 'graduated' == \
           decrypted_job_application_proof['requested_proof']['revealed_attrs']['attr4_referent'][1]
    assert '123-45-6789' == \
           decrypted_job_application_proof['requested_proof']['revealed_attrs']['attr5_referent'][1]

    assert 'Alice' == decrypted_job_application_proof['requested_proof']['self_attested_attrs']['attr1_referent']
    assert 'Garcia' == decrypted_job_application_proof['requested_proof']['self_attested_attrs']['attr2_referent']
    assert '123-45-6789' == decrypted_job_application_proof['requested_proof']['self_attested_attrs']['attr6_referent']

    assert await anoncreds.verifier_verify_proof(job_application_proof_request_json,
                                                 decrypted_job_application_proof_json,
                                                 schemas_json, claim_defs_json, revoc_regs_json)

    logger.info("==============================")
    logger.info("== Apply for the job with Acme - Getting Job-Certificate Claim ==")
    logger.info("------------------------------")

    logger.info("\"Acme\" -> Create \"Job-Certificate\" Claim Offer for Alice")
    job_certificate_claim_offer_json = \
        await anoncreds.issuer_create_claim_offer(acme_wallet, json.dumps(job_certificate_schema),
                                                  acme_issuer_did, alice_acme_did)

    logger.info("\"Acme\" -> Get key for Alice did")
    alice_acme_verkey = await did.key_for_did(pool_handle, acme_wallet, acme_alice_connection_response['did'])

    logger.info("\"Acme\" -> Authcrypt \"Job-Certificate\" Claim Offer for Alice")
    authcrypted_job_certificate_claim_offer = \
        await crypto.auth_crypt(acme_wallet, acme_alice_key, alice_acme_verkey, job_certificate_claim_offer_json)

    logger.info("\"Acme\" -> Send authcrypted \"Job-Certificate\" Claim Offer to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Job-Certificate\" Claim Offer from Acme")
    acme_alice_verkey, authdecrypted_job_certificate_claim_offer_json, authdecrypted_job_certificate_claim_offer = \
        await auth_decrypt(alice_wallet, alice_acme_key, authcrypted_job_certificate_claim_offer)

    logger.info("\"Alice\" -> Store \"Job-Certificate\" Claim Offer in Wallet")
    await anoncreds.prover_store_claim_offer(alice_wallet, authdecrypted_job_certificate_claim_offer_json)

    logger.info("\"Alice\" -> Get \"Job-Certificate\" Schema from Ledger")
    job_certificate_schema = \
        await get_schema(pool_handle, alice_acme_did, authdecrypted_job_certificate_claim_offer['schema_key'])

    logger.info("\"Alice\" -> Get \"Acme Job-Certificate\" Claim Definition from Ledger")
    acme_job_certificate_claim_def = await get_claim_def(pool_handle, alice_acme_did, job_certificate_schema,
                                                         authdecrypted_job_certificate_claim_offer['issuer_did'])

    logger.info("\"Alice\" -> Create and store in Wallet \"Job-Certificate\" Claim Request for Acme")
    job_certificate_claim_request_json = \
        await anoncreds.prover_create_and_store_claim_req(alice_wallet, alice_acme_did,
                                                          authdecrypted_job_certificate_claim_offer_json,
                                                          json.dumps(acme_job_certificate_claim_def),
                                                          alice_master_secret_name)

    logger.info("\"Alice\" -> Authcrypt \"Job-Certificate\" Claim Request for Acme")
    authcrypted_job_certificate_claim_request_json = \
        await crypto.auth_crypt(alice_wallet, alice_acme_key, acme_alice_verkey,
                                job_certificate_claim_request_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Job-Certificate\" Claim Request to Acme")

    logger.info("\"Acme\" -> Authdecrypt \"Job-Certificate\" Claim Request from Alice")
    alice_acme_verkey, authdecrypted_job_certificate_claim_request_json, _ = \
        await auth_decrypt(acme_wallet, acme_alice_key, authcrypted_job_certificate_claim_request_json)

    logger.info("\"Acme\" -> Create \"Job-Certificate\" Claim for Alice")
    alice_job_certificate_claim_values_json = json.dumps({
        'first_name': ['Alice', '245712572474217942457235975012103335'],
        'last_name': ['Garcia', '312643218496194691632153761283356127'],
        'employee_status': ['Permanent', '2143135425425143112321314321'],
        'salary': ['2400', '2400'],
        'experience': ['10', '10']
    })

    _, job_certificate_claim_json = \
        await anoncreds.issuer_create_claim(acme_wallet, authdecrypted_job_certificate_claim_request_json,
                                            alice_job_certificate_claim_values_json, -1)

    logger.info("\"Acme\" ->  Authcrypt \"Job-Certificate\" Claim for Alice")
    authcrypted_job_certificate_claim_json = \
        await crypto.auth_crypt(acme_wallet, acme_issuer_key, alice_acme_verkey,
                                job_certificate_claim_json.encode('utf-8'))

    logger.info("\"Acme\" ->  Send authcrypted \"Job-Certificate\" Claim to Alice")

    logger.info("\"Alice\" -> Authdecrypted \"Job-Certificate\" Claim from Acme")
    _, authdecrypted_job_certificate_claim_json, _ = \
        await auth_decrypt(alice_wallet, alice_acme_key, authcrypted_job_certificate_claim_json)

    logger.info("\"Alice\" -> Store \"Job-Certificate\" Claim")
    await anoncreds.prover_store_claim(alice_wallet, authdecrypted_job_certificate_claim_json, None)

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
        'requested_attrs': {
            'attr1_referent': {
                'name': 'employee_status',
                'restrictions': [{'issuer_did': acme_issuer_did, 'schema_key': job_certificate_schema_key}]
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'attr_name': 'salary',
                'p_type': '>=',
                'value': 2000,
                'restrictions': [{'issuer_did': acme_issuer_did, 'schema_key': job_certificate_schema_key}]
            },
            'predicate2_referent': {
                'attr_name': 'experience',
                'p_type': '>=',
                'value': 1,
                'restrictions': [{'issuer_did': acme_issuer_did, 'schema_key': job_certificate_schema_key}]
            }
        }
    })

    logger.info("\"Thrift\" -> Get key for Alice did")
    alice_thrift_verkey = await did.key_for_did(pool_handle, thrift_wallet, thrift_alice_connection_response['did'])

    logger.info("\"Thrift\" -> Authcrypt \"Loan-Application-Basic\" Proof Request for Alice")
    authcrypted_apply_loan_proof_request_json = \
        await crypto.auth_crypt(thrift_wallet, thrift_alice_key, alice_thrift_verkey,
                                apply_loan_proof_request_json.encode('utf-8'))

    logger.info("\"Thrift\" -> Sentd authcrypted \"Loan-Application-Basic\" Proof Request to Alice")

    logger.info("\"Alice\" -> Authdecrypt \"Loan-Application-Basic\" Proof Request from Thrift")
    thrift_alice_verkey, authdecrypted_apply_loan_proof_request_json, _ = \
        await auth_decrypt(alice_wallet, alice_thrift_key, authcrypted_apply_loan_proof_request_json)

    logger.info("\"Alice\" -> Get claims for \"Loan-Application-Basic\" Proof Request")
    claims_json_for_apply_loan_proof_request = \
        await anoncreds.prover_get_claims_for_proof_req(alice_wallet, authdecrypted_apply_loan_proof_request_json)
    claims_for_apply_loan_proof_request = json.loads(claims_json_for_apply_loan_proof_request)

    claim_for_attr1 = claims_for_apply_loan_proof_request['attrs']['attr1_referent'][0]
    claim_for_predicate1 = claims_for_apply_loan_proof_request['predicates']['predicate1_referent'][0]
    claim_for_predicate2 = claims_for_apply_loan_proof_request['predicates']['predicate2_referent'][0]

    claims_for_apply_loan_proof = {claim_for_attr1['referent']: claim_for_attr1,
                                   claim_for_predicate1['referent']: claim_for_predicate1,
                                   claim_for_predicate2['referent']: claim_for_predicate2}

    schemas_json, claim_defs_json, revoc_regs_json = \
        await get_entities_from_ledger(pool_handle, alice_thrift_did, claims_for_apply_loan_proof, 'Alice')

    logger.info("\"Alice\" -> Create \"Loan-Application-Basic\" Proof")
    apply_loan_requested_claims_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attrs': {
            'attr1_referent': [claim_for_attr1['referent'], True]
        },
        'requested_predicates': {
            'predicate1_referent': claim_for_predicate1['referent'],
            'predicate2_referent': claim_for_predicate2['referent']
        }
    })
    alice_apply_loan_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_apply_loan_proof_request_json,
                                            apply_loan_requested_claims_json, schemas_json, alice_master_secret_name,
                                            claim_defs_json, revoc_regs_json)

    logger.info("\"Alice\" -> Authcrypt \"Loan-Application-Basic\" Proof for Thrift")
    authcrypted_alice_apply_loan_proof_json = \
        await crypto.auth_crypt(alice_wallet, alice_thrift_key, thrift_alice_verkey,
                                alice_apply_loan_proof_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Loan-Application-Basic\" Proof to Thrift")

    logger.info("\"Thrift\" -> Authdecrypted \"Loan-Application-Basic\" Proof from Alice")
    _, authdecrypted_alice_apply_loan_proof_json, authdecrypted_alice_apply_loan_proof = \
        await auth_decrypt(thrift_wallet, thrift_alice_key, authcrypted_alice_apply_loan_proof_json)

    logger.info("\"Thrift\" -> Get Schemas, Claim Definitions and Revocation Registries from Ledger"
                " required for Proof verifying")

    schemas_json, claim_defs_json, revoc_regs_json = \
        await get_entities_from_ledger(pool_handle, thrift_did,
                                       authdecrypted_alice_apply_loan_proof['identifiers'], 'Thrift')

    logger.info("\"Thrift\" -> Verify \"Loan-Application-Basic\" Proof from Alice")
    assert 'Permanent' == authdecrypted_alice_apply_loan_proof['requested_proof']['revealed_attrs']['attr1_referent'][1]

    assert await anoncreds.verifier_verify_proof(apply_loan_proof_request_json,
                                                 authdecrypted_alice_apply_loan_proof_json,
                                                 schemas_json, claim_defs_json, revoc_regs_json)

    logger.info("==============================")

    logger.info("==============================")
    logger.info("== Apply for the loan with Thrift - Transcript and Job-Certificate proving  ==")
    logger.info("------------------------------")

    logger.info("\"Thrift\" -> Create \"Loan-Application-KYC\" Proof Request")
    apply_loan_kyc_proof_request_json = json.dumps({
        'nonce': '123432421212',
        'name': 'Loan-Application-KYC',
        'version': '0.1',
        'requested_attrs': {
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

    logger.info("\"Thrift\" -> Sentd authcrypted \"Loan-Application-KYC\" Proof Request to Alice")

    logger.info("\"Alice\" -> Authdecrypt \"Loan-Application-KYC\" Proof Request from Thrift")
    thrift_alice_verkey, authdecrypted_apply_loan_kyc_proof_request_json, _ = \
        await auth_decrypt(alice_wallet, alice_thrift_key, authcrypted_apply_loan_kyc_proof_request_json)

    logger.info("\"Alice\" -> Get claims for \"Loan-Application-KYC\" Proof Request")
    claims_json_for_apply_loan_kyc_proof_request = \
        await anoncreds.prover_get_claims_for_proof_req(alice_wallet, authdecrypted_apply_loan_kyc_proof_request_json)
    claims_for_apply_loan_kyc_proof_request = json.loads(claims_json_for_apply_loan_kyc_proof_request)

    claim_for_attr1 = claims_for_apply_loan_kyc_proof_request['attrs']['attr1_referent'][0]
    claim_for_attr2 = claims_for_apply_loan_kyc_proof_request['attrs']['attr2_referent'][0]
    claim_for_attr3 = claims_for_apply_loan_kyc_proof_request['attrs']['attr3_referent'][0]

    claims_for_apply_loan_kyc_proof = {claim_for_attr1['referent']: claim_for_attr1,
                                       claim_for_attr2['referent']: claim_for_attr2,
                                       claim_for_attr3['referent']: claim_for_attr3}

    schemas_json, claim_defs_json, revoc_regs_json = \
        await get_entities_from_ledger(pool_handle, alice_thrift_did, claims_for_apply_loan_kyc_proof, 'Alice')

    logger.info("\"Alice\" -> Create \"Loan-Application-KYC\" Proof")

    apply_loan_kyc_requested_claims_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attrs': {
            'attr1_referent': [claim_for_attr1['referent'], True],
            'attr2_referent': [claim_for_attr2['referent'], True],
            'attr3_referent': [claim_for_attr3['referent'], True]
        },
        'requested_predicates': {}
    })

    alice_apply_loan_kyc_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_apply_loan_kyc_proof_request_json,
                                            apply_loan_kyc_requested_claims_json, schemas_json,
                                            alice_master_secret_name, claim_defs_json, revoc_regs_json)

    logger.info("\"Alice\" -> Authcrypt \"Loan-Application-KYC\" Proof for Thrift")
    authcrypted_alice_apply_loan_kyc_proof_json = \
        await crypto.auth_crypt(alice_wallet, alice_thrift_key, thrift_alice_verkey,
                                alice_apply_loan_kyc_proof_json.encode('utf-8'))

    logger.info("\"Alice\" -> Send authcrypted \"Loan-Application-KYC\" Proof to Thrift")

    logger.info("\"Thrift\" -> Authdecrypted \"Loan-Application-KYC\" Proof from Alice")
    _, authdecrypted_alice_apply_loan_kyc_proof_json, authdecrypted_alice_apply_loan_kyc_proof = \
        await auth_decrypt(thrift_wallet, thrift_alice_key, authcrypted_alice_apply_loan_kyc_proof_json)

    logger.info("\"Thrift\" -> Get Schemas, Claim Definitions and Revocation Registries from Ledger"
                " required for Proof verifying")

    schemas_json, claim_defs_json, revoc_regs_json = \
        await get_entities_from_ledger(pool_handle, thrift_did,
                                       authdecrypted_alice_apply_loan_kyc_proof['identifiers'], 'Thrift')

    logger.info("\"Thrift\" -> Verify \"Loan-Application-KYC\" Proof from Alice")
    assert 'Alice' == authdecrypted_alice_apply_loan_kyc_proof['requested_proof']['revealed_attrs']['attr1_referent'][1]
    assert 'Garcia' == \
           authdecrypted_alice_apply_loan_kyc_proof['requested_proof']['revealed_attrs']['attr2_referent'][1]
    assert '123-45-6789' == \
           authdecrypted_alice_apply_loan_kyc_proof['requested_proof']['revealed_attrs']['attr3_referent'][1]

    assert await anoncreds.verifier_verify_proof(apply_loan_kyc_proof_request_json,
                                                 authdecrypted_alice_apply_loan_kyc_proof_json,
                                                 schemas_json, claim_defs_json, revoc_regs_json)

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
        await wallet.create_wallet(pool_name, to_wallet_name, None, None, None)
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
    schema_request = await ledger.build_schema_request(_did, json.dumps(schema))
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, _did, schema_request)


async def send_claim_def(pool_handle, wallet_handle, _did, claim_def_json):
    claim_def = json.loads(claim_def_json)
    claim_def_request = await ledger.build_claim_def_txn(_did, claim_def['ref'], claim_def['signature_type'],
                                                         json.dumps(claim_def['data']))
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, _did, claim_def_request)


async def get_schema(pool_handle, _did, schema_key):
    get_schema_data = json.dumps({
        'name': schema_key['name'],
        'version': schema_key['version']
    })
    get_schema_request = await ledger.build_get_schema_request(_did, schema_key['did'], get_schema_data)
    get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
    return json.loads(get_schema_response)['result']


async def get_claim_def(pool_handle, _did, schema, issuer_did):
    get_claim_def_request = await ledger.build_get_claim_def_txn(_did, schema['seqNo'], 'CL', issuer_did)
    get_claim_def_response = await ledger.submit_request(pool_handle, get_claim_def_request)
    return json.loads(get_claim_def_response)['result']


async def get_entities_from_ledger(pool_handle, _did, identifiers, actor):
    schemas = {}
    claim_defs = {}
    rev_regs = {}
    for referent, item in identifiers.items():
        logger.info("\"{}\" -> Get Schema from Ledger".format(actor))
        received_schema = await get_schema(pool_handle, _did, item['schema_key'])
        schemas[referent] = received_schema

        logger.info("\"{}\" -> Get Claim Definition from Ledger".format(actor))
        claim_defs[referent] = \
            await get_claim_def(pool_handle, _did, received_schema, item['issuer_did'])

        if 'rev_reg_seq_no' in item:
            pass  # TODO Get Revocation registries

    return json.dumps(schemas), json.dumps(claim_defs), json.dumps(rev_regs)


async def auth_decrypt(wallet_handle, key, message):
    from_verkey, decrypted_message_json = await crypto.auth_decrypt(wallet_handle, key, message)
    decrypted_message_json = decrypted_message_json.decode("utf-8")
    decrypted_message = json.loads(decrypted_message_json)
    return from_verkey, decrypted_message_json, decrypted_message
