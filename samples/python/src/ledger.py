import json
import time

from indy import ledger, did, wallet, pool
from src.utils import get_pool_genesis_txn_path, run_coroutine
import logging

logger = logging.getLogger(__name__)


async def demo():
    logger.info("Ledger sample -> started")

    pool_name = 'pool1'
    wallet_name = 'walle1t'
    seed_trustee1 = "000000000000000000000000Trustee1"
    pool_genesis_txn_path = get_pool_genesis_txn_path(pool_name)

    # 1. Create ledger config from genesis txn file
    pool_config = json.dumps({"genesis_txn": str(pool_genesis_txn_path)})
    await pool.create_pool_ledger_config(pool_name, pool_config)

    # 2. Open pool ledger
    pool_handle = await pool.open_pool_ledger(pool_name, None)

    # 3. Create Trustee Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)

    # 4. Create New DID
    (new_did, new_verkey) = await did.create_and_store_my_did(wallet_handle, "{}")

    # 5. Create DID from Trustee1 seed
    (trustee_did, _) = await did.create_and_store_my_did(wallet_handle, json.dumps({"seed": seed_trustee1}))

    # 6. Prepare and send NYM transaction
    nym_txn_req = await ledger.build_nym_request(trustee_did, new_did, new_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_txn_req)

    # 7. Prepare and send GET_NYM request
    get_nym_txn_req = await ledger.build_get_nym_request(trustee_did, new_did)
    get_nym_txn_resp = await ledger.submit_request(pool_handle, get_nym_txn_req)

    get_nym_txn_resp = json.loads(get_nym_txn_resp)

    assert get_nym_txn_resp['result']['dest'] == new_did

    # 8. Close wallet and pool
    await wallet.close_wallet(wallet_handle)
    await pool.close_pool_ledger(pool_handle)

    # 10. Delete wallets
    await wallet.delete_wallet(wallet_name, None)

    # 11. Delete pool ledger config
    await pool.delete_pool_ledger_config(pool_name)

    logger.info("Ledger sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
