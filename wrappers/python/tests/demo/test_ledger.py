import json

import pytest

from indy import ledger, did, wallet, pool


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_ledger_demo_works(pool_name, pool_genesis_txn_path, seed_trustee1, pool_genesis_txn_file, path_home):
    # 1. Create ledger config from genesis txn file
    pool_config = json.dumps({"genesis_txn": str(pool_genesis_txn_path)})
    await pool.create_pool_ledger_config(pool_name, pool_config)

    # 2. Open pool ledger
    pool_handle = await pool.open_pool_ledger(pool_name, None)

    # 3. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, 'my_wallet', None, None, None)
    my_wallet_handle = await wallet.open_wallet('my_wallet', None, None)

    # 4. Create Their Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, 'their_wallet', None, None, None)
    their_wallet_handle = await wallet.open_wallet('their_wallet', None, None)

    # 5. Create My DID
    (my_did, my_verkey) = await did.create_and_store_my_did(my_wallet_handle, "{}")

    # 6. Create Their DID from Trustee1 seed
    (their_did, their_verkey) = await did.create_and_store_my_did(their_wallet_handle,
                                                                  json.dumps({"seed": seed_trustee1}))

    # 7. Store Their DID
    await did.store_their_did(my_wallet_handle, json.dumps({'did': their_did, 'verkey': their_verkey}))

    # 8. Prepare and send NYM transaction
    nym_txn_req = await ledger.build_nym_request(their_did, my_did, None, None, None)
    await ledger.sign_and_submit_request(pool_handle, their_wallet_handle, their_did, nym_txn_req)

    # 9. Prepare and send GET_NYM request
    get_nym_txn_req = await ledger.build_get_nym_request(their_did, my_did)
    get_nym_txn_resp = await ledger.submit_request(pool_handle, get_nym_txn_req)

    get_nym_txn_resp = json.loads(get_nym_txn_resp)

    assert get_nym_txn_resp['result']['dest'] == my_did

    # 10. Close wallets and pool
    await wallet.close_wallet(their_wallet_handle)
    await wallet.close_wallet(my_wallet_handle)
    await pool.close_pool_ledger(pool_handle)
