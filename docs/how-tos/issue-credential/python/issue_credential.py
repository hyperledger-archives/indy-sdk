"""
This sample is extensions of "write_schema_and_cred_def.py"

Shows how to issue a credential as a Trust Anchor which has created a Cred Definition
for an existing Schema.

After Trust Anchor has successfully created and stored a Cred Definition using Anonymous Credentials,
Prover's wallet is created and opened, and used to generate Prover's Master Secret.
After that, Trust Anchor generates Credential Offer for given Cred Definition, using Prover's DID
Prover uses Credential Offer to create Credential Request
Trust Anchor then uses Prover's Credential Request to issue a Credential.
Finally, Prover stores Credential in its wallet.
"""

import asyncio
import json
import pprint

from indy import pool, ledger, wallet, did, anoncreds
from indy.error import ErrorCode, IndyError

from utils import open_wallet, get_pool_genesis_txn_path, PROTOCOL_VERSION

pool_name = 'pool'
issuer_wallet_config = json.dumps({"id": "issuer_wallet"})
issuer_wallet_credentials = json.dumps({"key": "issuer_wallet_key"})
genesis_file_path = get_pool_genesis_txn_path(pool_name)

def print_log(value_color="", value_noncolor=""):
    """set the colors for text."""
    HEADER = '\033[92m'
    ENDC = '\033[0m'
    print(HEADER + value_color + ENDC + str(value_noncolor))

async def issue_credential():
    try:
        await pool.set_protocol_version(PROTOCOL_VERSION)

        # 1.
        print_log('\n1. opening a new local pool ledger configuration that will be used '
                  'later when connecting to ledger.\n')
        pool_config = json.dumps({'genesis_txn': str(genesis_file_path)})
        try:
            await pool.create_pool_ledger_config(config_name=pool_name, config=pool_config)
        except IndyError as ex:
            if ex.error_code == ErrorCode.PoolLedgerConfigAlreadyExistsError:
                pass

        # 2.
        print_log('\n2. Open pool ledger and get the handle from libindy\n')
        pool_handle = await pool.open_pool_ledger(config_name=pool_name, config=None)

        # 3.
        print_log('\n3. Creating Issuer wallet and opening it to get the handle.\n')
        try:
            await wallet.create_wallet(config=issuer_wallet_config,credentials=issuer_wallet_credentials)
        except IndyError as err:
            if err.error_code == ErrorCode.WalletAlreadyExistsError:
                pass        
        issuer_wallet_handle = await wallet.open_wallet(issuer_wallet_config, issuer_wallet_credentials)

        # 4.
        print_log('\n4. Generating and storing steward DID and verkey\n')
        steward_seed = '000000000000000000000000Steward1'
        did_json = json.dumps({'seed': steward_seed})
        steward_did, steward_verkey = await did.create_and_store_my_did(issuer_wallet_handle, did_json)
        print_log('Steward DID: ', steward_did)
        print_log('Steward Verkey: ', steward_verkey)

        # 5.
        print_log('\n5. Generating and storing trust anchor DID and verkey\n')
        trust_anchor_did, trust_anchor_verkey = await did.create_and_store_my_did(issuer_wallet_handle, "{}")
        print_log('Trust anchor DID: ', trust_anchor_did)
        print_log('Trust anchor Verkey: ', trust_anchor_verkey)

        # 6.
        print_log('\n6. Building NYM request to add Trust Anchor to the ledger\n')
        nym_transaction_request = await ledger.build_nym_request(submitter_did=steward_did,
                                                                 target_did=trust_anchor_did,
                                                                 ver_key=trust_anchor_verkey,
                                                                 alias=None,
                                                                 role='TRUST_ANCHOR')
        print_log('NYM transaction request: ')
        pprint.pprint(json.loads(nym_transaction_request))

        # 7.
        print_log('\n7. Sending NYM request to the ledger\n')
        nym_transaction_response = await ledger.sign_and_submit_request(pool_handle=pool_handle,
                                                                        wallet_handle=issuer_wallet_handle,
                                                                        submitter_did=steward_did,
                                                                        request_json=nym_transaction_request)
        print_log('NYM transaction response: ')
        pprint.pprint(json.loads(nym_transaction_response))

        # 8.
        print_log('\n8. Issuer create Credential Schema\n')
        schema = {
            'name': 'gvt',
            'version': '1.0',
            'attributes': '["age", "sex", "height", "name"]'
        }
        issuer_schema_id, issuer_schema_json = await anoncreds.issuer_create_schema(steward_did, 
                                                                                schema['name'],
                                                                                schema['version'],
                                                                                schema['attributes'])
        print_log('Schema: ')
        pprint.pprint(issuer_schema_json)

        # 9.
        print_log('\n9. Build the SCHEMA request to add new schema to the ledger\n')
        schema_request = await ledger.build_schema_request(steward_did, issuer_schema_json)
        print_log('Schema request: ')
        pprint.pprint(json.loads(schema_request))

        # 10.
        print_log('\n10. Sending the SCHEMA request to the ledger\n')
        schema_response = \
            await ledger.sign_and_submit_request(pool_handle,
                                                 issuer_wallet_handle,
                                                 steward_did,
                                                 schema_request)
        print_log('Schema response:')
        pprint.pprint(json.loads(schema_response))

        # 11.
        print_log('\n11. Creating and storing Credential Definition using anoncreds as Trust Anchor, for the given Schema\n')
        cred_def_tag = 'TAG1'
        cred_def_type = 'CL'
        cred_def_config = json.dumps({"support_revocation": False})

        (cred_def_id, cred_def_json) = \
            await anoncreds.issuer_create_and_store_credential_def(issuer_wallet_handle,
                                                                   trust_anchor_did,
                                                                   issuer_schema_json,
                                                                   cred_def_tag,
                                                                   cred_def_type,
                                                                   cred_def_config)
        print_log('Credential definition: ')
        pprint.pprint(json.loads(cred_def_json))

        # 12.
        print_log('\n12. Creating Prover wallet and opening it to get the handle.\n')
        prover_did = 'VsKV7grR1BUE29mG2Fm2kX'
        prover_wallet_config = json.dumps({"id": "prover_wallet"})
        prover_wallet_credentials = json.dumps({"key": "prover_wallet_key"})
        try:
            await wallet.create_wallet(prover_wallet_config,prover_wallet_credentials)
        except IndyError as err:
            if err.error_code == ErrorCode.WalletAlreadyExistsError:
                pass        
        prover_wallet_handle = await wallet.open_wallet(prover_wallet_config, prover_wallet_credentials)

        # 13.
        print_log('\n13. Prover is creating Link Secret\n')
        prover_link_secret_name = 'link_secret'
        link_secret_id = await anoncreds.prover_create_master_secret(prover_wallet_handle,
                                                                     prover_link_secret_name)

        # 14.
        print_log('\n14. Issuer (Trust Anchor) is creating a Credential Offer for Prover\n')
        cred_offer_json = await anoncreds.issuer_create_credential_offer(issuer_wallet_handle,
                                                                         cred_def_id)
        print_log('Credential Offer: ')
        pprint.pprint(json.loads(cred_offer_json))

        # 15.
        print_log('\n15. Prover creates Credential Request for the given credential offer\n')
        (cred_req_json, cred_req_metadata_json) = \
            await anoncreds.prover_create_credential_req(prover_wallet_handle,
                                                         prover_did,
                                                         cred_offer_json,
                                                         cred_def_json,
                                                         prover_link_secret_name)
        print_log('Credential Request: ')
        pprint.pprint(json.loads(cred_req_json))

        # 16.
        print_log('\n16. Issuer (Trust Anchor) creates Credential for Credential Request\n')
        cred_values_json = json.dumps({
            "sex": {"raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233"},
            "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
            "height": {"raw": "175", "encoded": "175"},
            "age": {"raw": "28", "encoded": "28"}
        })
        (cred_json, _, _) = \
            await anoncreds.issuer_create_credential(issuer_wallet_handle,
                                                     cred_offer_json,
                                                     cred_req_json,
                                                     cred_values_json, None, None)
        print_log('Credential: ')
        pprint.pprint(json.loads(cred_json))

        # 17.
        print_log('\n17. Prover processes and stores received Credential\n')
        await anoncreds.prover_store_credential(prover_wallet_handle, None,
                                                cred_req_metadata_json,
                                                cred_json,
                                                cred_def_json, None)

        # 18.
        print_log('\n18. Closing both wallet_handles and pool\n')
        await wallet.close_wallet(issuer_wallet_handle)
        await wallet.close_wallet(prover_wallet_handle)
        await pool.close_pool_ledger(pool_handle)

        # 19.
        print_log('\n19. Deleting created wallet_handles\n')
        await wallet.delete_wallet(issuer_wallet_config, issuer_wallet_credentials)
        await wallet.delete_wallet(prover_wallet_config, prover_wallet_credentials)

        # 20.
        print_log('\n20. Deleting pool ledger config\n')
        await pool.delete_pool_ledger_config(pool_name)

    except IndyError as e:
        print('Error occurred: %s' % e)


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(issue_credential())
    loop.close()


if __name__ == '__main__':
    main()
