from indysdk import ledger, signus, wallet, pool
from indysdk.pool import open_pool_ledger

import pytest
import json

from tests.utils.pool import create_genesis_txn_file


@pytest.mark.asyncio
async def test_ledger_demo_works(cleanup_storage):
    # 1. Create ledger config from genesis txn file
    path = create_genesis_txn_file('pool_1.txn', None)
    pool_config = json.dumps({"genesis_txn": str(path)})
    await pool.create_pool_ledger_config('pool_1', pool_config)

    # 2. Open pool ledger
    pool_handle = await open_pool_ledger('pool_1', None)

    # 3. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet('pool_1', 'my_wallet', None, None, None)
    my_wallet_handle = await wallet.open_wallet('my_wallet', None, None)

    # 4. Create Their Wallet and Get Wallet Handle
    await wallet.create_wallet('pool_1', 'their_wallet', None, None, None)
    their_wallet_handle = await wallet.open_wallet('their_wallet', None, None)

    # 5. Create My DID
    (my_did, my_verkey, my_pk) = await signus.create_and_store_my_did(my_wallet_handle, "{}")

    # 6. Create Their DID from Trustee1 seed
    (their_did, their_verkey, their_pk) = \
        await signus.create_and_store_my_did(their_wallet_handle, '{"seed":"000000000000000000000000Trustee1"}')

    # 7. Store Their DID
    their_identity_json = json.dumps({
        'did': their_did,
        'pk': their_pk,
        'verkey': their_verkey
    })

    await signus.store_their_did(my_wallet_handle, their_identity_json)

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
