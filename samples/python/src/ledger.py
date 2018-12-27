import json
import time

from indy import ledger, did, wallet, pool
from src.utils import get_pool_genesis_txn_path, run_coroutine, PROTOCOL_VERSION
import logging

logger = logging.getLogger(__name__)


async def demo():
    logger.info("Ledger sample -> started")

    # Set protocol version 2 to work with Indy Node 1.4
    await pool.set_protocol_version(PROTOCOL_VERSION)

    trustee = {
        'seed': '000000000000000000000000Trustee1',
        'wallet_config': json.dumps({'id': 'trustee_wallet'}),
        'wallet_credentials': json.dumps({'key': 'trustee_wallet_key'}),
        'pool_name': 'trustee_pool',
    }

    # 1. Trustee open pool ledger
    trustee['genesis_txn_path'] = get_pool_genesis_txn_path(trustee['pool_name'])
    trustee['pool_config'] = json.dumps({"genesis_txn": str(trustee['genesis_txn_path'])})
    await pool.create_pool_ledger_config(trustee['pool_name'], trustee['pool_config'])

    trustee['pool'] = await pool.open_pool_ledger(trustee['pool_name'], None)

    # 2. Create Trustee Wallet and Get Wallet Handle
    await wallet.create_wallet(trustee['wallet_config'], trustee['wallet_credentials'])
    trustee['wallet'] = await wallet.open_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    # 3. Create Trustee DID
    (trustee['did'], trustee['verkey']) = \
        await did.create_and_store_my_did(trustee['wallet'], json.dumps({"seed": trustee['seed']}))

    # 4. User init
    user = {
        'wallet_config': json.dumps({'id': 'user_wallet'}),
        'wallet_credentials': json.dumps({'key': 'user_wallet_key'}),
        'pool_name': 'user_pool'
    }

    user['genesis_txn_path'] = get_pool_genesis_txn_path(user['pool_name'])
    user['pool_config'] = json.dumps({"genesis_txn": str(user['genesis_txn_path'])})
    await pool.create_pool_ledger_config(user['pool_name'], user['pool_config'])

    user['pool'] = await pool.open_pool_ledger(user['pool_name'], None)

    await wallet.create_wallet(user['wallet_config'], user['wallet_credentials'])
    user['wallet'] = await wallet.open_wallet(user['wallet_config'], user['wallet_credentials'])

    # 5. User create DID
    (user['did'], user['verkey']) = await did.create_and_store_my_did(user['wallet'], "{}")

    trustee['user_did'] = user['did']
    trustee['user_verkey'] = user['verkey']

    # 6. Trustee prepare and send NYM transaction for user
    nym_req = await ledger.build_nym_request(trustee['did'], trustee['user_did'], trustee['user_verkey'], None, None)
    await ledger.sign_and_submit_request(trustee['pool'], trustee['wallet'], trustee['did'], nym_req)

    # 7. User send ATTRIB transaction to Ledger
    attr_req = \
        await ledger.build_attrib_request(user['did'], user['did'], None, '{"endpoint":{"ha":"127.0.0.1:5555"}}', None)
    resp = await ledger.sign_and_submit_request(user['pool'], user['wallet'], user['did'], attr_req)

    assert json.loads(resp)['op'] == 'REPLY'

    # 8. Close and delete Trustee wallet
    await wallet.close_wallet(trustee['wallet'])
    await wallet.delete_wallet(trustee['wallet_config'], trustee['wallet_credentials'])

    # 9. Close and delete User wallet
    await wallet.close_wallet(user['wallet'])
    await wallet.delete_wallet(user['wallet_config'], user['wallet_credentials'])

    # 10. Close Trustee and User pools
    await pool.close_pool_ledger(trustee['pool'])
    await pool.close_pool_ledger(user['pool'])

    # 11 Delete pool ledger config
    await pool.delete_pool_ledger_config(trustee['pool_name'])
    await pool.delete_pool_ledger_config(user['pool_name'])

    logger.info("Ledger sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
