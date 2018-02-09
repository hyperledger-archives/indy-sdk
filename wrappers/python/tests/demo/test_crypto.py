from indy import did, wallet, pool, crypto

import pytest
import json


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_crypto_demo_works(pool_name, seed_trustee1, path_home, pool_genesis_txn_path, pool_genesis_txn_file):
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
    await did.create_and_store_my_did(my_wallet_handle, "{}")

    # 6. Create Their DID from Trustee1 seed
    (their_did, their_verkey) = await did.create_and_store_my_did(their_wallet_handle,
                                                                  json.dumps({"seed": seed_trustee1}))

    # 7. Store Their DID
    await did.store_their_did(my_wallet_handle, json.dumps({'did': their_did, 'verkey': their_verkey}))

    # 8. Their sign message
    message = json.dumps({
        "reqId": 1495034346617224651,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
        }
    })

    signature = await crypto.crypto_sign(their_wallet_handle, their_verkey, message)

    # 9. Their sign message
    assert await crypto.crypto_verify(their_verkey, message, signature)

    # 10. Close wallets
    await wallet.close_wallet(their_wallet_handle)
    await wallet.close_wallet(my_wallet_handle)
    await pool.close_pool_ledger(pool_handle)
