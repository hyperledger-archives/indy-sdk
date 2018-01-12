from indy import anoncreds, crypto, did, ledger, pool, wallet

import json
import logging

from src.utils import get_pool_genesis_txn_path

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


async def run():
    logger.info("Getting started -> started")

    logger.info("Open Pool Ledger")

    #  Create ledger config from genesis txn file
    pool_name = 'pool1'
    pool_genesis_txn_path = get_pool_genesis_txn_path(pool_name)
    pool_config = json.dumps({"genesis_txn": str(pool_genesis_txn_path)})
    await pool.create_pool_ledger_config(pool_name, pool_config)

    # Open pool ledger
    pool_handle = await pool.open_pool_ledger(pool_name, None)

    logger.info("==============================")
    logger.info("=== Getting Trust Anchor credentials for Faber, Acme, Thrift and Government  ==")

    logger.info("------------------------------")
    logger.info("\"Sovrin Steward\" -> Create wallet")

    # Create and Open Steward wallet
    steward_wallet_name = 'sovrin_steward_wallet'
    await wallet.create_wallet(pool_name, steward_wallet_name, None, None, None)
    steward_wallet = await wallet.open_wallet(steward_wallet_name, None, None)

    logger.info("\"Sovrin Steward\" -> Create and store in Wallet DID from seed")

    # Create and store Steward DID from seed
    steward_did_info = {'seed': '000000000000000000000000Steward1'}
    (steward_did, steward_key) = await did.create_and_store_my_did(steward_wallet, json.dumps(steward_did_info))

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Government Onboarding  ==")
    logger.info("------------------------------")

    logger.info("\"Sovrin Steward\" -> Create and store in Wallet \"Steward Government Pairwise\" DID")
    # Create and store Steward Government Pairwise DID
    (steward_government_did, steward_government_key) = await did.create_and_store_my_did(steward_wallet, "{}")

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Sovrin Steward Government Pairwise\" DID")
    # Build and Send Nym request for Steward Government Pairwise DID
    steward_government_nym_request = \
        await ledger.build_nym_request(steward_did, steward_government_did, steward_government_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, steward_government_nym_request)

    logger.info("\"Sovrin Steward\" -> Send connection request to Government "
                "with \"Sovrin Steward Government Pairwise\" DID and nonce")
    # Steward connection request to Government
    steward_government_connection_request = {
        'did': steward_government_did,
        'nonce': 123456789
    }

    logger.info("\"Government\" -> Create wallet")
    # Create and Open Government wallet
    government_wallet_name = 'government_wallet'
    await wallet.create_wallet(pool_name, government_wallet_name, None, None, None)
    government_wallet = await wallet.open_wallet(government_wallet_name, None, None)

    logger.info("\"Government\" -> Create and store in Wallet \"Government Sovrin Steward Pairwise\" DID")
    # Create and store Government Steward Pairwise DID
    (government_steward_did, government_steward_key) = await did.create_and_store_my_did(government_wallet, "{}")

    logger.info("\"Government\" -> Get key for did from \"Sovrin Steward\" connection request")
    # Government get key for did from connection request
    steward_government_verkey = \
        await did.key_for_did(pool_handle, government_wallet, steward_government_connection_request['did'])

    logger.info("\"Government\" -> Anoncrypt connection response for \"Sovrin Steward\" "
                "with \"Government Steward Pairwise\" DID, verkey and nonce")
    # Government connection response with Government Steward Pairwise DID, verkey and nonce
    steward_government_connection_response = json.dumps({
        'did': government_steward_did,
        'verkey': government_steward_key,
        'nonce': steward_government_connection_request['nonce']
    })
    # Government anoncrypt connection response
    anoncrypted_steward_government_connection_response = \
        await crypto.anon_crypt(steward_government_verkey, steward_government_connection_response.encode('utf-8'))

    logger.info("\"Government\" -> Send anoncrypted connection response to \"Sovrin Steward\""
                "with \"Government Sovrin Steward Pairwise\" DID, verkey and nonce")

    logger.info("\"Sovrin Steward\" -> Anondecrypt connection response from \"Government\"")
    # Sovrin Steward anondecrypt connection response from Government
    decrypted_steward_government_connection_response = \
        json.loads((await crypto.anon_decrypt(steward_wallet, steward_government_key,
                                              anoncrypted_steward_government_connection_response)).decode("utf-8"))

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Government Sovrin Steward Pairwise\" DID "
                "with Trust Anchor Role")
    # Build and Send Nym request for "Government Steward Pairwise" DID with Trust Anchor Role
    government_nym_request = await ledger.build_nym_request(steward_did,
                                                            decrypted_steward_government_connection_response['did'],
                                                            decrypted_steward_government_connection_response['verkey'],
                                                            None, 'TRUST_ANCHOR')
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, government_nym_request)

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Faber Onboarding  ==")
    logger.info("------------------------------")

    logger.info("\"Sovrin Steward\" -> Create and store in Wallet \"Steward Faber Pairwise\" DID")
    # Create and store Steward Faber Pairwise DID
    (steward_faber_did, steward_faber_key) = await did.create_and_store_my_did(steward_wallet, "{}")

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Sovrin Steward Faber Pairwise\" DID")
    # Build and Send Nym request for Steward Faber Pairwise DID
    steward_faber_nym_request = \
        await ledger.build_nym_request(steward_did, steward_faber_did, steward_faber_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, steward_faber_nym_request)

    logger.info("\"Sovrin Steward\" -> Send connection request to Faber "
                "with \"Sovrin Steward Faber Pairwise\" DID and nonce")
    # Steward connection request to Faber
    steward_faber_connection_request = {
        'did': steward_faber_did,
        'nonce': 987654321
    }

    logger.info("\"Faber\" -> Create wallet")
    # Create and Open Faber wallet
    faber_wallet_name = 'faber_wallet'
    await wallet.create_wallet(pool_name, faber_wallet_name, None, None, None)
    faber_wallet = await wallet.open_wallet(faber_wallet_name, None, None)

    logger.info("\"Faber\" -> Create and store in Wallet \"Faber Sovrin Steward Pairwise\" DID")
    # Create and store Faber Steward Pairwise DID
    (faber_steward_did, faber_steward_key) = await did.create_and_store_my_did(faber_wallet, "{}")

    logger.info("\"Faber\" -> Get key for did from \"Sovrin Steward\" connection request")
    # Faber get key for did from connection request
    steward_faber_verkey = \
        await did.key_for_did(pool_handle, faber_wallet, steward_faber_connection_request['did'])

    logger.info("\"Faber\" -> Anoncrypt connection response for \"Sovrin Steward\" "
                "with \"Fber Steward Pairwise\" DID, verkey and nonce")
    # Faber connection response with Faber Steward Pairwise DID, verkey and nonce
    steward_faber_connection_response = json.dumps({
        'did': faber_steward_did,
        'verkey': faber_steward_key,
        'nonce': steward_faber_connection_request['nonce']
    })
    # Faber anoncrypt connection response
    anoncrypted_steward_faber_connection_response = \
        await crypto.anon_crypt(steward_faber_verkey, steward_faber_connection_response.encode('utf-8'))

    logger.info("\"Faber\" -> Send anoncrypted connection response to \"Sovrin Steward\""
                "with \"Faber Sovrin Steward Pairwise\" DID, verkey and nonce")

    logger.info("\"Sovrin Steward\" -> Anondecrypt connection response from \"Faber\"")
    # Sovrin Steward anondecrypt connection response from Faber
    decrypted_steward_faber_connection_response = \
        json.loads((await crypto.anon_decrypt(steward_wallet, steward_faber_key,
                                              anoncrypted_steward_faber_connection_response)).decode("utf-8"))

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Faber Sovrin Steward Pairwise\" DID "
                "with Trust Anchor Role")
    # Build and Send Nym request for Faber Steward Pairwise DID with Trust Anchor Role
    faber_nym_request = await ledger.build_nym_request(steward_did,
                                                       decrypted_steward_faber_connection_response['did'],
                                                       decrypted_steward_faber_connection_response['verkey'],
                                                       None, 'TRUST_ANCHOR')
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, faber_nym_request)

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Acme Onboarding  ==")
    logger.info("------------------------------")

    logger.info("\"Sovrin Steward\" -> Create and store in Wallet \"Steward Acme Pairwise\" DID")
    # Create and store Steward Acme Pairwise DID
    (steward_acme_did, steward_acme_key) = await did.create_and_store_my_did(steward_wallet, "{}")

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Sovrin Steward Acme Pairwise\" DID")
    # Build and Send Nym request for Steward Acme Pairwise DID
    steward_acme_nym_request = \
        await ledger.build_nym_request(steward_did, steward_acme_did, steward_acme_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, steward_acme_nym_request)

    logger.info("\"Sovrin Steward\" -> Send connection request to Acme "
                "with \"Sovrin Steward Acme Pairwise\" DID and nonce")
    # Steward connection request to Acme
    steward_acme_connection_request = {
        'did': steward_acme_did,
        'nonce': 123459876
    }

    logger.info("\"Acme\" -> Create wallet")
    # Create and Open Acme wallet
    acme_wallet_name = 'acme_wallet'
    await wallet.create_wallet(pool_name, acme_wallet_name, None, None, None)
    acme_wallet = await wallet.open_wallet(acme_wallet_name, None, None)

    logger.info("\"Acme\" -> Create and store in Wallet \"Acme Sovrin Steward Pairwise\" DID")
    # Create and store Acme Steward Pairwise DID
    (acme_steward_did, acme_steward_key) = await did.create_and_store_my_did(acme_wallet, "{}")

    logger.info("\"Acme\" -> Get key for did from \"Sovrin Steward\" connection request")
    # Acme get key for did from connection request
    steward_acme_verkey = \
        await did.key_for_did(pool_handle, acme_wallet, steward_acme_connection_request['did'])

    logger.info("\"Acme\" -> Anoncrypt connection response for \"Sovrin Steward\" "
                "with \"Fber Steward Pairwise\" DID, verkey and nonce")
    # Acme connection response with Acme Steward Pairwise DID, verkey and nonce
    steward_acme_connection_response = json.dumps({
        'did': acme_steward_did,
        'verkey': acme_steward_key,
        'nonce': steward_acme_connection_request['nonce']
    })
    # Acme anoncrypt connection response
    anoncrypted_steward_acme_connection_response = \
        await crypto.anon_crypt(steward_acme_verkey, steward_acme_connection_response.encode('utf-8'))

    logger.info("\"Acme\" -> Send anoncrypted connection response to \"Sovrin Steward\""
                "with \"Acme Sovrin Steward Pairwise\" DID, verkey and nonce")

    logger.info("\"Sovrin Steward\" -> Anondecrypt connection response from \"Acme\"")
    # Sovrin Steward anondecrypt connection response from Acme
    decrypted_steward_acme_connection_response = \
        json.loads((await crypto.anon_decrypt(steward_wallet, steward_acme_key,
                                              anoncrypted_steward_acme_connection_response)).decode("utf-8"))

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Acme Sovrin Steward Pairwise\" DID "
                "with Trust Anchor Role")
    # Build and Send Nym request for Acme Steward Pairwise DID with Trust Anchor Role
    acme_nym_request = await ledger.build_nym_request(steward_did,
                                                      decrypted_steward_acme_connection_response['did'],
                                                      decrypted_steward_acme_connection_response['verkey'],
                                                      None, 'TRUST_ANCHOR')
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, acme_nym_request)

    logger.info("==============================")
    logger.info("== Getting Trust Anchor credentials - Thrift Onboarding  ==")
    logger.info("------------------------------")

    logger.info("\"Sovrin Steward\" -> Create and store in Wallet \"Steward Thrift Pairwise\" DID")
    # Create and store Steward Thrift Pairwise DID
    (steward_thrift_did, steward_thrift_key) = await did.create_and_store_my_did(steward_wallet, "{}")

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Sovrin Steward Thrift Pairwise\" DID")
    # Build and Send Nym request for Steward Thrift Pairwise DID
    steward_thrift_nym_request = \
        await ledger.build_nym_request(steward_did, steward_thrift_did, steward_thrift_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, steward_thrift_nym_request)

    logger.info("\"Sovrin Steward\" -> Send connection request to Thrift "
                "with \"Sovrin Steward Thrift Pairwise\" DID and nonce")
    # Steward connection request to Thrift
    steward_thrift_connection_request = {
        'did': steward_thrift_did,
        'nonce': 987654321
    }

    logger.info("\"Thrift\" -> Create wallet")
    # Create and Open Thrift wallet
    thrift_wallet_name = 'thrift_wallet'
    await wallet.create_wallet(pool_name, thrift_wallet_name, None, None, None)
    thrift_wallet = await wallet.open_wallet(thrift_wallet_name, None, None)

    logger.info("\"Thrift\" -> Create and store in Wallet \"Thrift Sovrin Steward Pairwise\" DID")
    # Create and store Acme Steward Pairwise DID
    (thrift_steward_did, thrift_steward_key) = await did.create_and_store_my_did(thrift_wallet, "{}")

    logger.info("\"Thrift\" -> Get key for did from \"Sovrin Steward\" connection request")
    # Thrift get key for did from connection request
    steward_thrift_verkey = \
        await did.key_for_did(pool_handle, thrift_wallet, steward_thrift_connection_request['did'])

    logger.info("\"Thrift\" -> Anoncrypt connection response for \"Sovrin Steward\" "
                "with \"Fber Steward Pairwise\" DID, verkey and nonce")
    # Thrift connection response with Thrift Steward Pairwise DID, verkey and nonce
    steward_thrift_connection_response = json.dumps({
        'did': thrift_steward_did,
        'verkey': thrift_steward_key,
        'nonce': steward_thrift_connection_request['nonce']
    })
    # Thrift anoncrypt connection response
    anoncrypted_steward_thrift_connection_response = \
        await crypto.anon_crypt(steward_thrift_verkey, steward_thrift_connection_response.encode('utf-8'))

    logger.info("\"Thrift\" -> Send anoncrypted connection response to \"Sovrin Steward\""
                "with \"Thrift Sovrin Steward Pairwise\" DID, verkey and nonce")

    logger.info("\"Sovrin Steward\" -> Anondecrypt connection response from \"Thrift\"")
    # Sovrin Steward anondecrypt connection response from Thrift
    decrypted_steward_thrift_connection_response = \
        json.loads((await crypto.anon_decrypt(steward_wallet, steward_thrift_key,
                                              anoncrypted_steward_thrift_connection_response)).decode("utf-8"))

    logger.info("\"Sovrin Steward\" -> Send Nym to Ledger for \"Thrift Sovrin Steward Pairwise\" DID "
                "with Trust Anchor Role")
    # Build and Send Nym request for Thrift Steward Pairwise DID with Trust Anchor Role
    thrift_nym_request = await ledger.build_nym_request(steward_did,
                                                        decrypted_steward_thrift_connection_response['did'],
                                                        decrypted_steward_thrift_connection_response['verkey'],
                                                        None, 'TRUST_ANCHOR')
    await ledger.sign_and_submit_request(pool_handle, steward_wallet, steward_did, thrift_nym_request)

    logger.info("==============================")
    logger.info("=== Claim Schemas Setup ==")
    logger.info("------------------------------")

    government_did = government_steward_did
    logger.info("\"Government\" -> Create and store in Wallet \"Government Schema Issuer\" DID")
    # Create and store Government Schema Issuer DID
    (government_issuer_did, government_issuer_key) = await did.create_and_store_my_did(government_wallet, "{}")

    logger.info("\"Government\" -> Send Nym to Ledger for \"Government Schema Issuer\" DID")
    # Build and Send Nym request for "Government Schema Issuer" DID
    government_nym_request = await ledger.build_nym_request(government_did,
                                                            government_issuer_did,
                                                            government_issuer_key,
                                                            None, None)
    await ledger.sign_and_submit_request(pool_handle, government_wallet, government_did, government_nym_request)

    logger.info("\"Government\" -> Send to Ledger \"Employment History\" Schema")
    # Employment History Schema
    employment_history_schema = {
        'name': 'Employment History',
        'version': '1.0',
        'attr_names': ['first_name', 'last_name', 'salary', 'employee_status', 'experience']
    }

    # Build and Send Schema request for Employment History Schema
    employment_history_schema_request = await ledger.build_schema_request(government_issuer_did,
                                                                          json.dumps(employment_history_schema))
    await ledger.sign_and_submit_request(pool_handle, government_wallet,
                                         government_issuer_did, employment_history_schema_request)

    logger.info("\"Government\" -> Send to Ledger \"HE Diploma\" Schema")
    # HE Diploma Schema
    he_diploma_schema = {
        'name': 'HE Diploma',
        'version': '1.0',
        'attr_names': ['first_name', 'last_name', 'phone_number', 'degree', 'status', 'ssn', 'average']
    }

    # Build and Send Schema request for HE Diploma Schema
    he_diploma_schema_request = await ledger.build_schema_request(government_issuer_did, json.dumps(he_diploma_schema))
    await ledger.sign_and_submit_request(pool_handle, government_wallet,
                                         government_issuer_did, he_diploma_schema_request)

    logger.info("==============================")
    logger.info("=== Faber Claim Definition Setup ==")
    logger.info("------------------------------")

    faber_did = faber_steward_did
    logger.info("\"Faber\" -> Create and store in Wallet \"Faber Claim Issuer\" DID")
    # Create and store Faber Claim Issuer DID
    (faber_issuer_did, faber_issuer_key) = await did.create_and_store_my_did(faber_wallet, "{}")

    logger.info("\"Faber\" -> Send Nym to Ledger for \"Faber Claim Issuer\" DID")
    # Build and Send Nym request for Faber Claim Issuer DID
    faber_nym_request = await ledger.build_nym_request(faber_did, faber_issuer_did, faber_issuer_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, faber_wallet, faber_did, faber_nym_request)

    logger.info("\"Faber\" -> Get \"HE Diploma\" Schema from Ledger")
    # Build and Send Get-Schema request for HE Diploma Schema
    # TODO: How did Faber get data and government_issuer_did?
    get_he_diploma_schema_data = {
        'name': 'HE Diploma',
        'version': '1.0'
    }
    get_he_diploma_schema_request = await ledger.build_get_schema_request(faber_issuer_did, government_issuer_did,
                                                                          json.dumps(get_he_diploma_schema_data))
    get_he_diploma_schema_response = await ledger.submit_request(pool_handle, get_he_diploma_schema_request)
    received_he_diploma_schema = json.loads(get_he_diploma_schema_response)['result']

    logger.info("\"Faber\" -> Create and store in Wallet \"Faber HE Diploma\" Claim Definition")
    # Create and store Faber HE Diploma Claim Definition
    faber_he_diploma_claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(faber_wallet, faber_issuer_did,
                                                          json.dumps(received_he_diploma_schema), 'CL', False)
    faber_he_diploma_claim_def = json.loads(faber_he_diploma_claim_def_json)

    logger.info("\"Faber\" -> Send ClaimDef to Ledger for \"Faber HE Diploma\" Claim Definition")
    # Build and Send ClaimDef request for Faber HE Diploma Claim Definition
    faber_he_diploma_claim_def_request = \
        await ledger.build_claim_def_txn(faber_issuer_did, faber_he_diploma_claim_def['ref'],
                                         faber_he_diploma_claim_def['signature_type'],
                                         json.dumps(faber_he_diploma_claim_def['data']))
    await ledger.sign_and_submit_request(pool_handle, faber_wallet, faber_issuer_did,
                                         faber_he_diploma_claim_def_request)

    logger.info("==============================")
    logger.info("=== Acme Claim Definition Setup ==")
    logger.info("------------------------------")

    acme_did = acme_steward_did
    logger.info("\"Acme\" -> Create and store in Wallet \"Acme Claim Issuer\" DID")
    # Create and store Acme Claim Issuer DID
    (acme_issuer_did, acme_issuer_key) = await did.create_and_store_my_did(acme_wallet, "{}")

    logger.info("\"Acme\" -> Send Nym to Ledger for \"Acme Claim Issuer\" DID")
    # Build and Send Nym request for "Acme Claim Issuer" DID
    acme_nym_request = await ledger.build_nym_request(acme_did, acme_issuer_did, acme_issuer_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, acme_wallet, acme_did, acme_nym_request)

    logger.info("\"Acme\" ->  Ge from Ledgert \"Employment History\" Schema")
    # Build and Send Get-Schema request for "Employment History" Schema
    # TODO: How did Acme get data and government_issuer_did?
    get_employment_history_schema_data = {
        'name': 'Employment History',
        'version': '1.0'
    }
    get_employment_history_schema_request = \
        await ledger.build_get_schema_request(acme_issuer_did, government_issuer_did,
                                              json.dumps(get_employment_history_schema_data))
    get_employment_history_schema_response = \
        await ledger.submit_request(pool_handle, get_employment_history_schema_request)
    received_employment_history_schema = json.loads(get_employment_history_schema_response)['result']

    logger.info("\"Acme\" -> Create and store in Wallet \"Acme Employment History\" Claim Definition")
    # Create and store "Acme Employment History" Claim Definition
    acme_employment_history_claim_def_json = \
        await anoncreds.issuer_create_and_store_claim_def(acme_wallet, acme_issuer_did,
                                                          json.dumps(received_employment_history_schema), 'CL', False)
    acme_employment_history_claim_def = json.loads(acme_employment_history_claim_def_json)

    logger.info("\"Acme\" -> Send ClaimDef to Ledger for \"Acme Employment History\" Claim Definition")
    # Build and Send ClaimDef request for "Acme Employment History" Claim Definition
    acme_employment_history_claim_def_request = \
        await ledger.build_claim_def_txn(acme_issuer_did, acme_employment_history_claim_def['ref'],
                                         acme_employment_history_claim_def['signature_type'],
                                         json.dumps(acme_employment_history_claim_def['data']))
    await ledger.sign_and_submit_request(pool_handle, acme_wallet, acme_issuer_did,
                                         acme_employment_history_claim_def_request)

    logger.info("==============================")
    logger.info("=== Getting HE Diploma with Faber ==")
    logger.info("==============================")
    logger.info("== Getting HE Diploma with Faber - Onboarding ==")
    logger.info("------------------------------")
    logger.info("Create and store \"Faber Alice Pairwise\" DID")

    # Create and store "Faber Alice Pairwise" DID
    (faber_alice_did, faber_alice_key) = await did.create_and_store_my_did(faber_wallet, "{}")

    logger.info("Send Nym for \"Faber Alice Pairwise\" DID")

    # Build and Send Nym request for "Faber Alice Pairwise" DID
    faber_did = faber_steward_did
    faber_alice_nym_request = \
        await ledger.build_nym_request(faber_did, faber_alice_did, faber_alice_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, faber_wallet, faber_did, faber_alice_nym_request)

    logger.info("Connection request with \"Faber Alice Pairwise\" DID and nonce")

    faber_alice_connection_request = {
        'did': faber_alice_did,
        'nonce': 111111111
    }

    logger.info("Create \"Alice\" wallet")

    # Create "Alice" wallet
    alice_wallet_name = 'alice_wallet'
    await wallet.create_wallet(pool_name, alice_wallet_name, None, None, None)
    alice_wallet = await wallet.open_wallet(alice_wallet_name, None, None)

    logger.info("Create and store \"Alice Faber Pairwise\" DID")

    # Create and store "Alice Faber Pairwise" DID
    (alice_faber_did, alice_faber_key) = await did.create_and_store_my_did(alice_wallet, "{}")

    logger.info("Anoncrypted connection response with \"Thrift Sovrin Steward Pairwise\" DID, verkey and nonce")

    # Get key for did in connection request
    faber_alice_verkey = await did.key_for_did(pool_handle, alice_wallet, faber_alice_connection_request['did'])

    # Anoncrypt connection response with "Alice Faber Pairwise" DID, verkey and nonce
    faber_alice_connection_response = json.dumps({
        'did': alice_faber_did,
        'verkey': alice_faber_key,
        'nonce': faber_alice_connection_request['nonce']
    })
    anoncrypted_faber_alice_connection_response = \
        await crypto.anon_crypt(faber_alice_verkey, faber_alice_connection_response.encode('utf-8'))

    logger.info("Send Nym for \"Alice Faber Pairwise\" DID")

    # Anondecrypt connection response from "Alice"
    decrypted_faber_alice_connection_response = \
        json.loads((await crypto.anon_decrypt(faber_wallet, faber_alice_key,
                                              anoncrypted_faber_alice_connection_response)).decode("utf-8"))

    # Build and Send Nym request for "Alice Faber Pairwise" DID
    alice_nym_request = await ledger.build_nym_request(faber_did,
                                                       decrypted_faber_alice_connection_response['did'],
                                                       decrypted_faber_alice_connection_response['verkey'],
                                                       None, None)
    await ledger.sign_and_submit_request(pool_handle, faber_wallet, faber_did, alice_nym_request)

    logger.info("== Getting HE Diploma with Faber - Getting HE Diploma Claim ==")
    logger.info("Send authcrypted \"Alice Faber HE Diploma\" Claim Offer")

    he_diploma_schema_key = {
        'name': he_diploma_schema['name'],
        'version': he_diploma_schema['version'],
        'did': government_issuer_did
    }

    # Faber Authcrypt "Alice Faber HE Diploma" Claim Offer
    alice_faber_he_diploma_claim_offer = {
        'issuer_did': faber_issuer_did,
        'schema_key': he_diploma_schema_key
    }
    authcrypted_faber_alice_he_diploma_claim_offer = \
        await crypto.auth_crypt(faber_wallet, faber_issuer_key, decrypted_faber_alice_connection_response['verkey'],
                                json.dumps(alice_faber_he_diploma_claim_offer).encode('utf-8'))

    logger.info("Store \"Alice Faber HE Diploma\" Claim Offer")

    # Alice Authdecrypt "Alice Faber HE Diploma" Claim Offer
    _, authdecrypted_faber_alice_he_diploma_claim_offer_json = \
        await crypto.auth_decrypt(alice_wallet, alice_faber_key, authcrypted_faber_alice_he_diploma_claim_offer)
    authdecrypted_faber_alice_he_diploma_claim_offer_json = \
        authdecrypted_faber_alice_he_diploma_claim_offer_json.decode("utf-8")
    authdecrypted_faber_alice_he_diploma_claim_offer = json.loads(authdecrypted_faber_alice_he_diploma_claim_offer_json)

    # Store "Alice Faber HE Diploma" Claim Offer
    await anoncreds.prover_store_claim_offer(alice_wallet, authdecrypted_faber_alice_he_diploma_claim_offer_json)

    logger.info("Create and store \"Alice\" Master Secret")

    # Create and store "Alice" Master Secret
    alice_master_secret_name = 'alice_master_secret'
    await anoncreds.prover_create_master_secret(alice_wallet, alice_master_secret_name)

    logger.info("Alice get \"Faber HE Diploma\" Claim Definition")

    alice_did = alice_faber_did

    # Build and Send GetSchema request for "HE Diploma" Schema
    get_he_diploma_schema_data = {
        'name': authdecrypted_faber_alice_he_diploma_claim_offer['schema_key']['name'],
        'version': authdecrypted_faber_alice_he_diploma_claim_offer['schema_key']['version']
    }
    get_he_diploma_schema_request = \
        await ledger.build_get_schema_request(alice_did,
                                              authdecrypted_faber_alice_he_diploma_claim_offer['schema_key']['did'],
                                              json.dumps(get_he_diploma_schema_data))
    get_he_diploma_schema_response = \
        await ledger.submit_request(pool_handle, get_he_diploma_schema_request)
    received_he_diploma_schema = json.loads(get_he_diploma_schema_response)['result']

    # Build and Send GetClaimDef request for "Faber HE Diploma" Claim Definition
    get_faber_he_diploma_claim_def_request = \
        await ledger.build_get_claim_def_txn(alice_did,
                                             received_he_diploma_schema['seqNo'],
                                             'CL',
                                             authdecrypted_faber_alice_he_diploma_claim_offer['issuer_did'])
    get_faber_he_diploma_claim_def_response = \
        await ledger.submit_request(pool_handle, get_faber_he_diploma_claim_def_request)
    received_faber_he_diploma_claim_def = json.loads(get_faber_he_diploma_claim_def_response)['result']

    logger.info("Create and store \"Alice Faber HE Diploma\" Claim Request")

    # Create and store "Alice Faber HE Diploma" Claim Request
    alice_he_diploma_claim_request_json = \
        await anoncreds.prover_create_and_store_claim_req(alice_wallet,
                                                          alice_did,
                                                          authdecrypted_faber_alice_he_diploma_claim_offer_json,
                                                          json.dumps(received_faber_he_diploma_claim_def),
                                                          alice_master_secret_name)

    logger.info("Send authcrypted \"Alice Faber HE Diploma\" Claim Request")

    # Get key for did in claim offer
    faber_alice_verkey = \
        await did.key_for_did(pool_handle, alice_wallet, authdecrypted_faber_alice_he_diploma_claim_offer['issuer_did'])

    # Authcrypt "Alice Faber HE Diploma" Claim Request
    authcrypted_alice_he_diploma_claim_request = \
        await crypto.auth_crypt(alice_wallet, alice_faber_key,
                                faber_alice_verkey,
                                alice_he_diploma_claim_request_json.encode('utf-8'))

    logger.info("Create \"Alice Faber HE Diploma\" Claim")

    # Authdecrypt "Alice Faber HE Diploma" Claim Request
    _, authdecrypted_alice_he_diploma_claim_request_json = \
        await crypto.auth_decrypt(faber_wallet, faber_issuer_key, authcrypted_alice_he_diploma_claim_request)
    authdecrypted_alice_he_diploma_claim_request_json = \
        authdecrypted_alice_he_diploma_claim_request_json.decode("utf-8")
    authdecrypted_alice_he_diploma_claim_request = json.loads(authdecrypted_alice_he_diploma_claim_request_json)

    # Create "Alice Faber HE Diploma" Claim
    alice_faber_he_diploma_claim_values = json.dumps({
        'first_name': ['Alice', '1139481716457488690172217916278103335'],
        'last_name': ['Garcia', '5321642780241790123587902456789123452'],
        'phone_number': ['123-45-6789', '2345673233145748869017221534567810333'],
        'degree': ['Bachelor of Science, Marketing', '12434523576212321'],
        'status': ['graduated', '2213454313412354'],
        'ssn': ['123-45-6789', '3124141231422543541'],
        'average': ['5', '5']
    })

    (_, alice_faber_he_diploma_claim_json) = \
        await anoncreds.issuer_create_claim(faber_wallet,
                                            authdecrypted_alice_he_diploma_claim_request_json,
                                            alice_faber_he_diploma_claim_values,
                                            -1)

    logger.info("Send authcrypted \"Alice Faber HE Diploma\" Claim")

    # Get key for did in claim offer
    received_alice_key = \
        await did.key_for_did(pool_handle, faber_wallet, authdecrypted_alice_he_diploma_claim_request['prover_did'])

    # Authcrypt "Alice Faber HE Diploma" Claim
    authcrypted_alice_faber_he_diploma_claim_json = \
        await crypto.auth_crypt(faber_wallet, faber_issuer_key, received_alice_key,
                                alice_faber_he_diploma_claim_json.encode('utf-8'))

    logger.info("Store \"Alice Faber HE Diploma\" Claim")

    # Alice Authdecrypt "Alice Faber HE Diploma" Claim
    _, authdecrypted_faber_alice_he_diploma_claim_json = \
        await crypto.auth_decrypt(alice_wallet, alice_faber_key, authcrypted_alice_faber_he_diploma_claim_json)
    authdecrypted_faber_alice_he_diploma_claim_json = \
        authdecrypted_faber_alice_he_diploma_claim_json.decode("utf-8")

    # Store "Alice Faber HE Diploma" Claim
    await anoncreds.prover_store_claim(alice_wallet, authdecrypted_faber_alice_he_diploma_claim_json, None)

    logger.info("=== Apply for the job with Acme ==")
    logger.info("== Apply for the job with Acme - Onboarding ==")
    logger.info("Create and store \"Acme Alice Pairwise\" DID")

    # Create and store "Acme Alice Pairwise" DID
    (acme_alice_did, acme_alice_key) = await did.create_and_store_my_did(acme_wallet, "{}")

    logger.info("Send Nym for \"Acme Alice Pairwise\" DID")

    # Build and Send Nym request for "Acme Alice Pairwise" DID
    acme_did = acme_steward_did
    acme_alice_nym_request = \
        await ledger.build_nym_request(acme_did, acme_alice_did, acme_alice_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, acme_wallet, acme_did, acme_alice_nym_request)

    logger.info("Connection request with \"Acme Alice Pairwise\" DID and nonce")

    acme_alice_connection_request = {
        'did': acme_alice_did,
        'nonce': 222222222
    }

    logger.info("Create and store \"Alice Acme Pairwise\" DID")

    # Create and store "Alice Acme Pairwise" DID
    (alice_acme_did, alice_acme_key) = await did.create_and_store_my_did(alice_wallet, "{}")

    logger.info("Anoncrypted connection response with \"Alice Acme Pairwise\" DID, verkey and nonce")

    # Get key for did in connection request
    acme_alice_verkey = await did.key_for_did(pool_handle, alice_wallet, acme_alice_connection_request['did'])

    # Anoncrypt connection response with "Alice Faber Pairwise" DID, verkey and nonce
    acme_alice_connection_response = json.dumps({
        'did': alice_acme_did,
        'verkey': alice_acme_key,
        'nonce': acme_alice_connection_request['nonce']
    })
    anoncrypted_acme_alice_connection_response = \
        await crypto.anon_crypt(acme_alice_verkey, acme_alice_connection_response.encode('utf-8'))

    logger.info("Send Nym for \"Alice Acme Pairwise\" DID")

    # Anondecrypt connection response from "Alice"
    decrypted_acme_alice_connection_response = \
        json.loads((await crypto.anon_decrypt(acme_wallet, acme_alice_key,
                                              anoncrypted_acme_alice_connection_response)).decode("utf-8"))

    # Build and Send Nym request for "Alice Acme Pairwise" DID
    alice_acme_nym_request = await ledger.build_nym_request(acme_did,
                                                            decrypted_acme_alice_connection_response['did'],
                                                            decrypted_acme_alice_connection_response['verkey'],
                                                            None, None)
    await ledger.sign_and_submit_request(pool_handle, acme_wallet, acme_did, alice_acme_nym_request)

    logger.info("== Apply for the job with Acme - HE Diploma proving ==")
    logger.info("Send authcrypted \"HE Diploma\" Proof Request")

    proof_req_json = json.dumps({
        'nonce': '1432422343242122312411212',
        'name': 'Job-Application',
        'version': '0.1',
        'requested_attrs': {
            'attr1_referent': {
                'name': 'first_name',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': he_diploma_schema_key}]
            },
            'attr2_referent': {
                'name': 'last_name',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': he_diploma_schema_key}]
            },
            'attr3_referent': {
                'name': 'degree',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': he_diploma_schema_key}]
            },
            'attr4_referent': {
                'name': 'status',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': he_diploma_schema_key}]
            },
            'attr5_referent': {
                'name': 'phone_number',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': he_diploma_schema_key}]
            },
            'attr6_referent': {
                'name': 'ssn',
                'restrictions': [{'issuer_did': faber_issuer_did, 'schema_key': he_diploma_schema_key}]
            }
        },
        'requested_predicates': {
            'predicate1_referent': {
                'attr_name': 'average',
                'p_type': '>=',
                'value': 4,
                'restrictions': [{'issuer_did': faber_issuer_did}]
            }
        }
    })

    # Authcrypt "HE Diploma" Proof Request
    authcrypted_he_diploma_acme_proof_request = \
        await crypto.auth_crypt(acme_wallet, acme_alice_key,
                                decrypted_acme_alice_connection_response['verkey'],
                                proof_req_json.encode('utf-8'))

    logger.info("Get claims for \"HE Diploma\" Proof Request")

    # Alice Authdecrypt "HE Diploma" Proof Request
    _, authdecrypted_he_diploma_acme_proof_request = \
        await crypto.auth_decrypt(alice_wallet, alice_acme_key, authcrypted_he_diploma_acme_proof_request)
    authdecrypted_he_diploma_acme_proof_request = \
        authdecrypted_he_diploma_acme_proof_request.decode("utf-8")

    # Get claims for "HE Diploma" Proof Request
    claim_for_proof_request_json = \
        await anoncreds.prover_get_claims_for_proof_req(alice_wallet,
                                                        authdecrypted_he_diploma_acme_proof_request)
    claims_for_proof_request = json.loads(claim_for_proof_request_json)

    claim_for_attr1 = claims_for_proof_request['attrs']['attr1_referent'][0]
    claim_for_attr2 = claims_for_proof_request['attrs']['attr2_referent'][0]
    claim_for_attr3 = claims_for_proof_request['attrs']['attr3_referent'][0]
    claim_for_attr4 = claims_for_proof_request['attrs']['attr4_referent'][0]
    claim_for_attr5 = claims_for_proof_request['attrs']['attr5_referent'][0]
    claim_for_attr6 = claims_for_proof_request['attrs']['attr6_referent'][0]
    claim_for_predicate1 = claims_for_proof_request['predicates']['predicate1_referent'][0]

    logger.info("Create \"Alice HE Diploma\" Proof")

    schemas_json = {}
    claim_defs_json = {}
    revoc_regs_json = {}

    claims_for_proof = {claim_for_attr1['referent']: claim_for_attr1,
                        claim_for_attr2['referent']: claim_for_attr2,
                        claim_for_attr3['referent']: claim_for_attr3,
                        claim_for_attr4['referent']: claim_for_attr4,
                        claim_for_attr5['referent']: claim_for_attr5,
                        claim_for_attr6['referent']: claim_for_attr6,
                        claim_for_predicate1['referent']: claim_for_predicate1}

    for referent, claim in claims_for_proof.items():
        # Get schemas required for proof building
        get_schema_data = {
            'name': claim['schema_key']['name'],
            'version': claim['schema_key']['version']
        }
        get_schema_request = \
            await ledger.build_get_schema_request(alice_did, claim['schema_key']['did'], json.dumps(get_schema_data))
        get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
        received_schema = json.loads(get_schema_response)['result']
        schemas_json[referent] = received_schema

        # Get claim defs required for proof building
        get_claim_def_request = \
            await ledger.build_get_claim_def_txn(alice_did,
                                                 received_schema['seqNo'],
                                                 'CL',
                                                 claim['issuer_did'])
        get_claim_def_response = await ledger.submit_request(pool_handle, get_claim_def_request)
        received_claim_def = json.loads(get_claim_def_response)['result']
        claim_defs_json[referent] = received_claim_def

        if 'rev_reg_seq_no' in claim:
            pass  # TODO Get Revocation registries

    # Create "Alice HE Diploma" Proof
    he_diploma_requested_claims_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attrs': {
            'attr1_referent': [claim_for_attr1['referent'], False],
            'attr2_referent': [claim_for_attr2['referent'], False],
            'attr3_referent': [claim_for_attr3['referent'], True],
            'attr4_referent': [claim_for_attr4['referent'], True],
            'attr5_referent': [claim_for_attr5['referent'], False],
            'attr6_referent': [claim_for_attr6['referent'], True]
        },
        'requested_predicates': {'predicate1_referent': claim_for_predicate1['referent']}
    })
    alice_he_diploma_acme_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_he_diploma_acme_proof_request,
                                            he_diploma_requested_claims_json, json.dumps(schemas_json),
                                            alice_master_secret_name, json.dumps(claim_defs_json),
                                            json.dumps(revoc_regs_json))

    # Authcrypt "Alice HE Diploma" Proof
    authcrypted_alice_he_diploma_acme_proof_json = \
        await crypto.auth_crypt(alice_wallet, alice_acme_key,
                                acme_alice_key,
                                alice_he_diploma_acme_proof_json.encode('utf-8'))

    logger.info("Verify \"Alice HE Diploma\" Proof")

    # Alice Authdecrypt "Alice HE Diploma" Proof Request
    _, authdecrypted_he_diploma_acme_proof_request = \
        await crypto.auth_decrypt(acme_wallet, acme_alice_key, authcrypted_alice_he_diploma_acme_proof_json)
    authdecrypted_he_diploma_acme_proof_request = \
        authdecrypted_he_diploma_acme_proof_request.decode("utf-8")

    # Фсьу get schemas for proof verifying
    alice_he_diploma_acme_proof = json.loads(alice_he_diploma_acme_proof_json)

    schemas_json = {}
    claim_defs_json = {}
    revoc_regs_json = {}

    for referent, identifier in alice_he_diploma_acme_proof['identifiers'].items():
        # Get required verifying
        get_he_diploma_schema_data = {
            'name': identifier['schema_key']['name'],
            'version': identifier['schema_key']['version']
        }
        get_he_diploma_schema_request = \
            await ledger.build_get_schema_request(acme_did, identifier['schema_key']['did'],
                                                  json.dumps(get_he_diploma_schema_data))
        get_he_diploma_schema_response = await ledger.submit_request(pool_handle, get_he_diploma_schema_request)
        received_he_diploma_schema = json.loads(get_he_diploma_schema_response)['result']
        schemas_json[referent] = received_he_diploma_schema

        # Get  required claim defs for proof verifying
        get_he_diploma_claim_def_request = \
            await ledger.build_get_claim_def_txn(acme_did,
                                                 received_he_diploma_schema['seqNo'],
                                                 'CL',
                                                 identifier['issuer_did'])
        get_he_diploma_claim_def_response = await ledger.submit_request(pool_handle, get_he_diploma_claim_def_request)
        received_he_diploma_claim_def = json.loads(get_he_diploma_claim_def_response)['result']
        claim_defs_json[referent] = received_he_diploma_claim_def

        if 'rev_reg_seq_no' in identifier:
            pass  # TODO Get Revocation registries

    # Verify "Alice HE Diploma" Proof
    # Check revealed attributes
    assert 'Bachelor of Science, Marketing' == \
           alice_he_diploma_acme_proof['requested_proof']['revealed_attrs']['attr3_referent'][1]
    assert 'graduated' == \
           alice_he_diploma_acme_proof['requested_proof']['revealed_attrs']['attr4_referent'][1]
    assert '123-45-6789' == \
           alice_he_diploma_acme_proof['requested_proof']['revealed_attrs']['attr6_referent'][1]

    # Check unrevealed attributes
    assert 'attr1_referent' in alice_he_diploma_acme_proof['requested_proof']['unrevealed_attrs']
    assert 'attr2_referent' in alice_he_diploma_acme_proof['requested_proof']['unrevealed_attrs']
    assert 'attr5_referent' in alice_he_diploma_acme_proof['requested_proof']['unrevealed_attrs']

    # Check proof
    assert await anoncreds.verifier_verify_proof(proof_req_json, authdecrypted_he_diploma_acme_proof_request,
                                                 json.dumps(schemas_json), json.dumps(claim_defs_json),
                                                 json.dumps(revoc_regs_json))

    logger.info("== Apply for the job with Acme - Getting Employment History Claim ==")
    logger.info("Send authcrypted \"Alice Acme Employment History\" Claim Offer")

    employment_history_schema_key = {
        'name': employment_history_schema['name'],
        'version': employment_history_schema['version'],
        'did': government_issuer_did
    }

    # Acme Authcrypt "Alice Acme Employment History" Claim Offer
    alice_acme_employment_history_claim_offer = {
        'issuer_did': acme_issuer_did,
        'schema_key': employment_history_schema_key
    }
    authcrypted_acme_alice_employment_history_claim_offer = \
        await crypto.auth_crypt(acme_wallet, acme_issuer_key, decrypted_acme_alice_connection_response['verkey'],
                                json.dumps(alice_acme_employment_history_claim_offer).encode('utf-8'))

    logger.info("Store \"Alice Acme Employment History\" Claim Offer")

    # Alice Authdecrypt "Alice Acme Employment History" Claim Offer
    _, authdecrypted_acme_alice_employment_history_claim_offer_json = \
        await crypto.auth_decrypt(alice_wallet, alice_acme_key, authcrypted_acme_alice_employment_history_claim_offer)
    authdecrypted_acme_alice_employment_history_claim_offer_json = \
        authdecrypted_acme_alice_employment_history_claim_offer_json.decode("utf-8")
    authdecrypted_acme_alice_employment_history_claim_offer = \
        json.loads(authdecrypted_acme_alice_employment_history_claim_offer_json)

    # Store "Alice Acme Employment History" Claim Offer
    await anoncreds.prover_store_claim_offer(alice_wallet, authdecrypted_acme_alice_employment_history_claim_offer_json)

    logger.info("Alice get \"Acme Employment History\" Claim Definition")

    # Build and Send GetSchema request for "HE Diploma" Schema
    get_employment_history_schema_data = {
        'name': authdecrypted_acme_alice_employment_history_claim_offer['schema_key']['name'],
        'version': authdecrypted_acme_alice_employment_history_claim_offer['schema_key']['version']
    }
    get_employment_history_schema_request = \
        await ledger.build_get_schema_request(alice_did,
                                              authdecrypted_acme_alice_employment_history_claim_offer['schema_key'][
                                                  'did'],
                                              json.dumps(get_employment_history_schema_data))
    get_employment_history_schema_response = \
        await ledger.submit_request(pool_handle, get_employment_history_schema_request)
    received_employment_history_schema = json.loads(get_employment_history_schema_response)['result']

    # Build and Send GetClaimDef request for "Faber HE Diploma" Claim Definition
    get_acme_employment_history_claim_def_request = \
        await ledger.build_get_claim_def_txn(alice_did,
                                             received_employment_history_schema['seqNo'],
                                             'CL',
                                             authdecrypted_acme_alice_employment_history_claim_offer['issuer_did'])
    get_acme_employment_history_claim_def_response = \
        await ledger.submit_request(pool_handle, get_acme_employment_history_claim_def_request)
    received_acme_employment_history_claim_def = json.loads(get_acme_employment_history_claim_def_response)['result']

    logger.info("Create and store \"Alice Acme Employment History\" Claim Request")

    # Create and store "Alice Acme Employment History" Claim Request
    alice_acme_employment_history_claim_request_json = \
        await anoncreds.prover_create_and_store_claim_req(alice_wallet,
                                                          alice_did,
                                                          authdecrypted_acme_alice_employment_history_claim_offer_json,
                                                          json.dumps(received_acme_employment_history_claim_def),
                                                          alice_master_secret_name)

    logger.info("Send authcrypted \"Alice Acme Employment History\" Claim Request")

    # Get key for did in claim offer
    acme_alice_verkey = await did.key_for_did(pool_handle, alice_wallet,
                                              authdecrypted_acme_alice_employment_history_claim_offer['issuer_did'])

    # Authcrypt "Alice Acme Employment History" Claim Request
    authcrypted_alice_acme_employment_history_claim_request = \
        await crypto.auth_crypt(alice_wallet, alice_acme_key,
                                acme_alice_verkey,
                                alice_acme_employment_history_claim_request_json.encode('utf-8'))

    logger.info("Create \"Alice Acme Employment History\" Claim")

    # Authdecrypt "Alice Acme Employment History" Claim Request
    _, authdecrypted_alice_acme_employment_history_claim_request_json = \
        await crypto.auth_decrypt(acme_wallet, acme_issuer_key, authcrypted_alice_acme_employment_history_claim_request)
    authdecrypted_alice_acme_employment_history_claim_request_json = \
        authdecrypted_alice_acme_employment_history_claim_request_json.decode("utf-8")
    authdecrypted_alice_acme_employment_history_claim_request = \
        json.loads(authdecrypted_alice_acme_employment_history_claim_request_json)

    # Create "Alice Acme Employment History" Claim
    alice_acme_employment_history_claim_values = json.dumps({
        'first_name': ['Alice', '245712572474217942457235975012103335'],
        'last_name': ['Garcia', '312643218496194691632153761283356127'],
        'employee_status': ['Permanent', '2143135425425143112321314321'],
        'salary': ['2400', '2400'],
        'experience': ['10', '10']
    })

    (_, acme_alice_employment_history_claim_json) = \
        await anoncreds.issuer_create_claim(acme_wallet,
                                            authdecrypted_alice_acme_employment_history_claim_request_json,
                                            alice_acme_employment_history_claim_values,
                                            -1)

    logger.info("Send authcrypted \"Alice Acme Employment History\" Claim")

    # Get key for did in claim offer
    received_alice_key = await did.key_for_did(pool_handle, acme_wallet,
                                               authdecrypted_alice_acme_employment_history_claim_request['prover_did'])

    # Authcrypt "Alice Acme Employment History" Claim
    authcrypted_acme_alice_employment_history_claim_json = \
        await crypto.auth_crypt(acme_wallet, acme_issuer_key, received_alice_key,
                                acme_alice_employment_history_claim_json.encode('utf-8'))

    logger.info("Store \"Alice Acme Employment History\" Claim")

    # Alice Authdecrypt "Alice Acme Employment History" Claim
    _, authdecrypted_acme_alice_employment_history_claim_json = \
        await crypto.auth_decrypt(alice_wallet, alice_faber_key, authcrypted_acme_alice_employment_history_claim_json)
    authdecrypted_acme_alice_employment_history_claim_json = \
        authdecrypted_acme_alice_employment_history_claim_json.decode("utf-8")

    # Store "Alice Acme Employment History" Claim
    await anoncreds.prover_store_claim(alice_wallet, authdecrypted_acme_alice_employment_history_claim_json, None)

    logger.info("=== Apply for the loan with Thrift ==")
    logger.info("== Apply for the loan with Thrift - Onboarding ==")
    logger.info("Create and store \"Thrift Alice Pairwise\" DID")

    # Create and store "Thrift Alice Pairwise" DID
    (thrift_alice_did, thrift_alice_key) = await did.create_and_store_my_did(thrift_wallet, "{}")

    logger.info("Send Nym for \"Thrift Alice Pairwise\" DID")

    # Build and Send Nym request for "Thrift Alice Pairwise" DID
    thrift_did = thrift_steward_did
    thrift_alice_nym_request = \
        await ledger.build_nym_request(thrift_did, thrift_alice_did, thrift_alice_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, thrift_wallet, thrift_did, thrift_alice_nym_request)

    logger.info("Connection request with \"Thrift Alice Pairwise\" DID and nonce")

    thrift_alice_connection_request = {
        'did': thrift_alice_did,
        'nonce': 333333333
    }

    logger.info("Create and store \"Alice Thrift Pairwise\" DID")

    # Create and store "Alice Thrift Pairwise" DID
    (alice_thrift_did, alice_thrift_key) = await did.create_and_store_my_did(alice_wallet, "{}")

    logger.info("Anoncrypted connection response with \"Alice Thrift Pairwise\" DID, verkey and nonce")

    # Get key for did in connection request
    thrift_alice_verkey = await did.key_for_did(pool_handle, alice_wallet, thrift_alice_connection_request['did'])

    # Anoncrypt connection response with "Alice Thrift Pairwise" DID, verkey and nonce
    thrift_alice_connection_response = json.dumps({
        'did': alice_thrift_did,
        'verkey': alice_thrift_key,
        'nonce': thrift_alice_connection_request['nonce']
    })
    anoncrypted_thrift_alice_connection_response = \
        await crypto.anon_crypt(thrift_alice_verkey, thrift_alice_connection_response.encode('utf-8'))

    logger.info("Send Nym for \"Alice Thrift Pairwise\" DID")

    # Anondecrypt connection response from "Alice"
    decrypted_thrift_alice_connection_response = \
        json.loads((await crypto.anon_decrypt(thrift_wallet, thrift_alice_key,
                                              anoncrypted_thrift_alice_connection_response)).decode("utf-8"))

    # Build and Send Nym request for "Alice Thrift Pairwise" DID
    alice_thrift_nym_request = await ledger.build_nym_request(thrift_did,
                                                              decrypted_thrift_alice_connection_response['did'],
                                                              decrypted_thrift_alice_connection_response['verkey'],
                                                              None, None)
    await ledger.sign_and_submit_request(pool_handle, thrift_wallet, thrift_did, alice_thrift_nym_request)

    logger.info("== Apply for the loan with Thrift - HE Diploma and Employment History proving  ==")
    logger.info("Send authcrypted \"HE Diploma and Employment History\" Proof Requestt")

    apply_for_loan_proof_request_json = json.dumps({
        'nonce': '123432421212',
        'name': 'Loan-Application',
        'version': '0.1',
        'requested_attrs': {
            'attr1_referent': {'name': 'first_name'},
            'attr2_referent': {'name': 'last_name'},
            'attr3_referent': {
                'name': 'degree',
                'restrictions': [{
                    'issuer_did': faber_issuer_did,
                    'schema_key': he_diploma_schema_key
                }]
            },
            'attr4_referent': {
                'name': 'employee_status',
                'restrictions': [{
                    'issuer_did': acme_issuer_did,
                    'schema_key': employment_history_schema_key
                }]
            },
            'attr5_referent': {'name': 'ssn'}
        },
        'requested_predicates': {
            'predicate1_referent': {
                'attr_name': 'salary',
                'p_type': '>=',
                'value': 2000,
                'restrictions': [{'issuer_did': acme_issuer_did}]
            },
            'predicate2_referent': {
                'attr_name': 'experience',
                'p_type': '>=',
                'value': 6,
                'restrictions': [{'issuer_did': acme_issuer_did, 'schema_key': employment_history_schema_key}]
            }
        }
    })

    # Authcrypt "HE Diploma and Employment History" Proof Request
    authcrypted_apply_for_loan_proof_request_json = \
        await crypto.auth_crypt(thrift_wallet, thrift_alice_key,
                                decrypted_thrift_alice_connection_response['verkey'],
                                apply_for_loan_proof_request_json.encode('utf-8'))

    logger.info("Get claims for \"HE Diploma and Employment History\" Proof Request")

    # Alice Authdecrypt "HE Diploma and Employment History" Proof Request
    _, authdecrypted_apply_for_loan_proof_request_json = \
        await crypto.auth_decrypt(alice_wallet, alice_thrift_key, authcrypted_apply_for_loan_proof_request_json)
    authdecrypted_apply_for_loan_proof_request_json = \
        authdecrypted_apply_for_loan_proof_request_json.decode("utf-8")

    # Get claims for "HE Diploma and Employment History" Proof Request
    claims_json_for_apply_for_loan_proof_request = \
        await anoncreds.prover_get_claims_for_proof_req(alice_wallet,
                                                        authdecrypted_apply_for_loan_proof_request_json)
    claims_for_apply_for_loan_proof_request = json.loads(claims_json_for_apply_for_loan_proof_request)

    claim_for_attr1 = claims_for_apply_for_loan_proof_request['attrs']['attr1_referent'][0]
    claim_for_attr2 = claims_for_apply_for_loan_proof_request['attrs']['attr2_referent'][0]
    claim_for_attr3 = claims_for_apply_for_loan_proof_request['attrs']['attr3_referent'][0]
    claim_for_attr4 = claims_for_apply_for_loan_proof_request['attrs']['attr4_referent'][0]
    claim_for_attr5 = claims_for_apply_for_loan_proof_request['attrs']['attr5_referent'][0]
    claim_for_predicate1 = claims_for_apply_for_loan_proof_request['predicates']['predicate1_referent'][0]
    claim_for_predicate2 = claims_for_apply_for_loan_proof_request['predicates']['predicate2_referent'][0]

    logger.info("Create \"Alice HE Diploma\" Proof")

    schemas_json = {}
    claim_defs_json = {}
    revoc_regs_json = {}

    claims_for_proof = {claim_for_attr1['referent']: claim_for_attr1,
                        claim_for_attr2['referent']: claim_for_attr2,
                        claim_for_attr3['referent']: claim_for_attr3,
                        claim_for_attr4['referent']: claim_for_attr4,
                        claim_for_attr5['referent']: claim_for_attr5,
                        claim_for_predicate1['referent']: claim_for_predicate1,
                        claim_for_predicate2['referent']: claim_for_predicate2}

    for referent, claim in claims_for_proof.items():
        # Get schemas required for proof building
        get_schema_data = {
            'name': claim['schema_key']['name'],
            'version': claim['schema_key']['version']
        }
        get_schema_request = \
            await ledger.build_get_schema_request(alice_did, claim['schema_key']['did'], json.dumps(get_schema_data))
        get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
        received_schema = json.loads(get_schema_response)['result']
        schemas_json[referent] = received_schema

        # Get claim defs required for proof building
        get_claim_def_request = \
            await ledger.build_get_claim_def_txn(alice_did,
                                                 received_schema['seqNo'],
                                                 'CL',
                                                 claim['issuer_did'])
        get_claim_def_response = await ledger.submit_request(pool_handle, get_claim_def_request)
        received_claim_def = json.loads(get_claim_def_response)['result']
        claim_defs_json[referent] = received_claim_def

        if 'rev_reg_seq_no' in claim:
            pass  # TODO Get Revocation registries

    # Create "HE Diploma and Employment History" Proof
    apply_for_loan_requested_claims_json = json.dumps({
        'self_attested_attributes': {},
        'requested_attrs': {
            'attr1_referent': [claim_for_attr1['referent'], True],
            'attr2_referent': [claim_for_attr2['referent'], True],
            'attr3_referent': [claim_for_attr3['referent'], True],
            'attr4_referent': [claim_for_attr4['referent'], True],
            'attr5_referent': [claim_for_attr5['referent'], True]
        },
        'requested_predicates': {
            'predicate1_referent': claim_for_predicate1['referent'],
            'predicate2_referent': claim_for_predicate2['referent']
        }
    })
    alice_apply_for_loan_proof_json = \
        await anoncreds.prover_create_proof(alice_wallet, authdecrypted_apply_for_loan_proof_request_json,
                                            apply_for_loan_requested_claims_json, json.dumps(schemas_json),
                                            alice_master_secret_name, json.dumps(claim_defs_json),
                                            json.dumps(revoc_regs_json))

    # Authcrypt "Alice Faber HE Diploma and Alice Acme Employment History" Proof
    authcrypted_alice_apply_for_loan_proof_json = \
        await crypto.auth_crypt(alice_wallet, alice_thrift_key,
                                thrift_alice_key,
                                alice_apply_for_loan_proof_json.encode('utf-8'))

    logger.info("Verify \"Alice Faber HE Diploma and Alice Acme Employment History\" Proof")

    # Alice Authdecrypt "Alice HE Diploma" Proof Request
    _, authdecrypted_alice_apply_for_loan_proof_json = \
        await crypto.auth_decrypt(thrift_wallet, thrift_alice_key, authcrypted_alice_apply_for_loan_proof_json)
    authdecrypted_alice_apply_for_loan_proof_json = \
        authdecrypted_alice_apply_for_loan_proof_json.decode("utf-8")

    # Thrift get schemas for proof verifying
    authdecrypted_alice_apply_for_loan_proof = json.loads(authdecrypted_alice_apply_for_loan_proof_json)

    schemas_json = {}
    claim_defs_json = {}
    revoc_regs_json = {}

    for referent, identifier in authdecrypted_alice_apply_for_loan_proof['identifiers'].items():
        # Get required verifying
        get_schema_data = {
            'name': identifier['schema_key']['name'],
            'version': identifier['schema_key']['version']
        }
        get_schema_request = \
            await ledger.build_get_schema_request(thrift_did, identifier['schema_key']['did'],
                                                  json.dumps(get_schema_data))
        get_schema_response = await ledger.submit_request(pool_handle, get_schema_request)
        received_schema = json.loads(get_schema_response)['result']
        schemas_json[referent] = received_schema

        # Get claim defs required for proof verifying
        get_claim_def_request = \
            await ledger.build_get_claim_def_txn(thrift_did,
                                                 received_schema['seqNo'],
                                                 'CL',
                                                 identifier['issuer_did'])
        get_claim_def_response = await ledger.submit_request(pool_handle, get_claim_def_request)
        received_claim_def = json.loads(get_claim_def_response)['result']
        claim_defs_json[referent] = received_claim_def

        if 'rev_reg_seq_no' in identifier:
            pass  # TODO Get Revocation registries

    # Verify "Alice Faber HE Diploma and Alice Acme Employment History" Proof
    assert 'Alice' == authdecrypted_alice_apply_for_loan_proof['requested_proof']['revealed_attrs']['attr1_referent'][1]
    assert 'Garcia' == \
           authdecrypted_alice_apply_for_loan_proof['requested_proof']['revealed_attrs']['attr2_referent'][1]
    assert 'Bachelor of Science, Marketing' == \
           authdecrypted_alice_apply_for_loan_proof['requested_proof']['revealed_attrs']['attr3_referent'][1]
    assert 'Permanent' == \
           authdecrypted_alice_apply_for_loan_proof['requested_proof']['revealed_attrs']['attr4_referent'][1]
    assert '123-45-6789' == \
           authdecrypted_alice_apply_for_loan_proof['requested_proof']['revealed_attrs']['attr5_referent'][1]

    assert await anoncreds.verifier_verify_proof(apply_for_loan_proof_request_json,
                                                 authdecrypted_alice_apply_for_loan_proof_json,
                                                 json.dumps(schemas_json), json.dumps(claim_defs_json),
                                                 json.dumps(revoc_regs_json))

    logger.info("Close and Delete \"Sovrin Steward\" wallet")
    # Close and Delete "Sovrin Steward" wallet
    await wallet.close_wallet(steward_wallet)
    await wallet.delete_wallet(steward_wallet_name, None)

    logger.info("Close and Delete \"Sovrin Government\" wallet")
    # Close and Delete "Sovrin Government" wallet
    await wallet.close_wallet(government_wallet)
    await wallet.delete_wallet(government_wallet_name, None)

    logger.info("Close and Delete \"Sovrin Faber\" wallet")
    # Close and Delete "Sovrin Faber" wallet
    await wallet.close_wallet(faber_wallet)
    await wallet.delete_wallet(faber_wallet_name, None)

    logger.info("Close and Delete \"Sovrin Acme\" wallet")
    # Close and Delete "Sovrin Acme" wallet
    await wallet.close_wallet(acme_wallet)
    await wallet.delete_wallet(acme_wallet_name, None)

    logger.info("Close and Delete \"Sovrin Thrift\" wallet")
    # Close and Delete "Sovrin Thrift" wallet
    await wallet.close_wallet(thrift_wallet)
    await wallet.delete_wallet(thrift_wallet_name, None)

    logger.info("Close and Delete \"Alice\" wallet")
    # Close and Delete "Alice" wallet
    await wallet.close_wallet(alice_wallet)
    await wallet.delete_wallet(alice_wallet_name, None)

    logger.info("Close and Delete pool")
    # Close and Delete pool
    await pool.close_pool_ledger(pool_handle)
    await pool.delete_pool_ledger_config(pool_name)

    logger.info("Getting started -> done")
