import time
import json
import logging

from src.utils import get_pool_genesis_txn_path, run_coroutine, PROTOCOL_VERSION


logger = logging.getLogger(__name__)

from indy import ledger, did, wallet, pool, anoncreds


async def open_pool(target_pool):
    target_pool['genesis_txn_path'] = get_pool_genesis_txn_path(target_pool['pool_name'])
    target_pool['pool_config'] = json.dumps({"genesis_txn": str(target_pool['genesis_txn_path'])})
    await pool.create_pool_ledger_config(target_pool['pool_name'], target_pool['pool_config'])
    target_pool['pool'] = await pool.open_pool_ledger(target_pool['pool_name'], None)


async def setup_identity(identity, trustee):
    (identity['did'], identity['verkey']) = await did.create_and_store_my_did(identity['wallet'], "{}")
    nym_req = await ledger.build_nym_request(trustee['did'], identity['did'], identity['verkey'], None,
                                             identity['role'])
    await ledger.sign_and_submit_request(identity['pool'], trustee['wallet'], trustee['did'], nym_req)


async def demo():
    logger.info("endorser sample -> started")

    # Set protocol version 2 to work with Indy Node 1.4
    await pool.set_protocol_version(PROTOCOL_VERSION)

    # Open pool
    target_pool = {
        'pool_name': 'trustee_pool'
    }

    await open_pool(target_pool)

    author = {
        'wallet_config': json.dumps({'id': 'author_wallet'}),
        'wallet_credentials': json.dumps({'key': 'author_wallet_key'}),
        'pool': target_pool['pool'],
        'role': None
    }
    endorser = {
        'wallet_config': json.dumps({"id": "endorser_wallet"}),
        'wallet_credentials': json.dumps({"key": "endorser_wallet_key"}),
        'pool': target_pool['pool'],
        'role': 'ENDORSER'
    }
    trustee = {
        'seed': '000000000000000000000000Trustee1',
        'wallet_config': json.dumps({'id': 'trustee_wallet'}),
        'wallet_credentials': json.dumps({'key': 'trustee_wallet_key'}),
        'pool': target_pool['pool']
    }

    await wallet.create_wallet(author['wallet_config'], author['wallet_credentials'])
    author['wallet'] = await wallet.open_wallet(author['wallet_config'], author['wallet_credentials'])

    await wallet.create_wallet(endorser['wallet_config'], endorser['wallet_credentials'])
    endorser['wallet'] = await wallet.open_wallet(endorser['wallet_config'], endorser['wallet_credentials'])

    await wallet.create_wallet(trustee['wallet_config'], trustee['wallet_credentials'])
    trustee['wallet'] = await wallet.open_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    # Trustee Create DID
    (trustee['did'], trustee['verkey']) = \
        await did.create_and_store_my_did(trustee['wallet'], json.dumps({"seed": trustee['seed']}))

    # Author Create DID
    await setup_identity(author, trustee)

    #  Endorser Create DID
    await setup_identity(endorser, trustee)

    # Write SCHEMA from author with endorser
    # Endorser Workflow
    schema = {
        'name': 'gvt',
        'version': '1.0',
        'attributes': '["age", "sex", "height", "name"]'
    }
    author['schema_id'], author['schema'] = await anoncreds.issuer_create_schema(author['did'], schema['name'],
                                                                                 schema['version'],
                                                                                 schema['attributes'])

    # 1. Transaction Author builds Schema Request
    author['schema_req'] = await ledger.build_schema_request(author['did'], author['schema'])

    # 2. Transaction Author appends Endorser's DID into the request
    author['schema_req'] = await ledger.append_request_endorser(author['schema_req'], endorser['did'])

    # 3. Transaction Author signs the request with the added endorser field
    author['schema_req'] = await ledger.multi_sign_request(author['wallet'], author['did'], author['schema_req'])

    # 4. Transaction Author sends the request to the Endorser (out of scope).
    endorser['schema_req'] = author['schema_req']

    # 5. Transaction Endorser signs the request
    endorser['schema_req'] = await ledger.multi_sign_request(endorser['wallet'], endorser['did'],
                                                             endorser['schema_req'])

    # 6. Transaction Endorser sends the request
    resp = await ledger.submit_request(target_pool['pool'], endorser['schema_req'])
    resp = json.loads(resp)
    assert resp['op'] == 'REPLY'
    assert resp['result']['txn']['metadata']['endorser'] == endorser['did']

    time.sleep(1)

    # Author checks that Schema was added
    author['get_schema_req'] = await ledger.build_get_schema_request(author['did'], author['schema_id'])
    response = await ledger.submit_request(target_pool['pool'], author['get_schema_req'])
    response = json.loads(response)
    assert response['op'] == 'REPLY'

    await pool.close_pool_ledger(target_pool['pool'])
    await pool.delete_pool_ledger_config(target_pool['pool_name'])

    await wallet.close_wallet(trustee['wallet'])
    await wallet.delete_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    await wallet.close_wallet(author['wallet'])
    await wallet.delete_wallet(author['wallet_config'], author['wallet_credentials'])

    await wallet.close_wallet(endorser['wallet'])
    await wallet.delete_wallet(endorser['wallet_config'], endorser['wallet_credentials'])

    logger.info("endorser sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
