import time
import json
import logging

from src.utils import get_pool_genesis_txn_path, run_coroutine, PROTOCOL_VERSION

logger = logging.getLogger(__name__)

from indy import ledger, did, wallet, pool, anoncreds

async def demo():
    logger.info("endorser sample -> started")

    author = {
        'wallet_config': json.dumps({'id': 'author_wallet'}),
        'wallet_credentials': json.dumps({'key': 'author_wallet_key'})
    }
    endorser = {
        'wallet_config': json.dumps({"id": "endorser_wallet"}),
        'wallet_credentials': json.dumps({"key": "endorser_wallet_key"})
    }
    trustee = {
        'seed': '000000000000000000000000Trustee1',
        'wallet_config': json.dumps({'id': 'trustee_wallet'}),
        'wallet_credentials': json.dumps({'key': 'trustee_wallet_key'}),
    }

    # Set protocol version 2 to work with Indy Node 1.4
    await pool.set_protocol_version(PROTOCOL_VERSION)

    await wallet.create_wallet(author['wallet_config'], author['wallet_credentials'])
    author['wallet'] = await wallet.open_wallet(author['wallet_config'], author['wallet_credentials'])

    await wallet.create_wallet(endorser['wallet_config'], endorser['wallet_credentials'])
    endorser['wallet'] = await wallet.open_wallet(endorser['wallet_config'], endorser['wallet_credentials'])

    await wallet.create_wallet(trustee['wallet_config'], trustee['wallet_credentials'])
    trustee['wallet'] = await wallet.open_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    # 2. Signer Create DID
    (author['did'], author['verkey']) = await did.create_and_store_my_did(author['wallet'], "{}")

    # 3. Verifier Create DID
    (endorser['did'], endorser['verkey']) = await did.create_and_store_my_did(endorser['wallet'], "{}")

    # 4. Trustee Create DID
    (trustee['did'], trustee['verkey']) = \
        await did.create_and_store_my_did(trustee['wallet'], json.dumps({"seed": trustee['seed']}))
    
    # 5. Open pool
    target_pool = {
        'pool_name': 'trustee_pool'
    }

    target_pool['genesis_txn_path'] = get_pool_genesis_txn_path(target_pool['pool_name'])
    target_pool['pool_config'] = json.dumps({"genesis_txn": str(target_pool['genesis_txn_path'])})
    await pool.create_pool_ledger_config(target_pool['pool_name'], target_pool['pool_config'])
    target_pool['pool'] = await pool.open_pool_ledger(target_pool['pool_name'], None)

    # 5. Write DIDs to the ledger
    author_nym_req = await ledger.build_nym_request(trustee['did'], author['did'], author['verkey'], None, None)
    resp = await ledger.sign_and_submit_request(target_pool['pool'], trustee['wallet'], trustee['did'], author_nym_req)

    endorser_nym_req = \
        await ledger.build_nym_request(trustee['did'], endorser['did'], endorser['verkey'], None, "ENDORSER")
    resp = await ledger.sign_and_submit_request(target_pool['pool'], trustee['wallet'], trustee['did'], endorser_nym_req)

    # 6. Write SCHEMA from author with endorser
    schema = {
        'name': 'gvt',
        'version': '1.0',
        'attributes': '["age", "sex", "height", "name"]'
    }
    author['schema_id'], author['schema'] = await anoncreds.issuer_create_schema(author['did'], schema['name'],
                                                                                 schema['version'],
                                                                                 schema['attributes'])

    author_schema_req = await ledger.build_schema_request(author['did'], author['schema'])
    author_schema_req_with_endorser = await ledger.append_request_endorser(author_schema_req, endorser['did'])
    author_schema_req_with_endorser_signed_author = \
        await ledger.multi_sign_request(author['wallet'], author['did'], author_schema_req_with_endorser)
    author_schema_req_with_endorser_signed = \
        await ledger.multi_sign_request(endorser['wallet'], endorser['did'], author_schema_req_with_endorser_signed_author)

    resp = await ledger.submit_request(target_pool['pool'], author_schema_req_with_endorser_signed)
    assert json.loads(resp)['op'] == 'REPLY'

    await pool.close_pool_ledger(target_pool['pool'])
    await pool.delete_pool_ledger_config(target_pool['pool_name'])

    await wallet.close_wallet(trustee['wallet'])
    await wallet.delete_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    await wallet.close_wallet(author['wallet'])
    await wallet.delete_wallet(author['wallet_config'], author['wallet_credentials'])

    await wallet.close_wallet(endorser['wallet'])
    await wallet.delete_wallet(endorser['wallet_config'], endorser['wallet_credentials'])

    logger.info("Crypto sample -> completed")

if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
