from indy import signus, wallet

import pytest
import json


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_signus_demo_works(pool_name, seed_trustee1, path_home):
    # 1. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, 'my_wallet', None, None, None)
    my_wallet_handle = await wallet.open_wallet('my_wallet', None, None)

    # 2. Create Their Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, 'their_wallet', None, None, None)
    their_wallet_handle = await wallet.open_wallet('their_wallet', None, None)

    # 3. Create My DID
    await signus.create_and_store_my_did(my_wallet_handle, "{}")

    # 4. Create Their DID from Trustee1 seed
    (their_did, their_verkey, their_pk) = \
        await signus.create_and_store_my_did(their_wallet_handle, json.dumps({"seed": seed_trustee1}))

    # 5. Store Their DID
    their_identity_json = json.dumps({
        'did': their_did,
        'pk': their_pk,
        'verkey': their_verkey
    })

    await signus.store_their_did(my_wallet_handle, their_identity_json)

    # 6. Their sign message
    message = json.dumps({
        "reqId": 1495034346617224651,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
        }
    })

    signature = await signus.sign(their_wallet_handle, their_did, message)

    # 7. Their sign message
    assert await signus.verify_signature(my_wallet_handle, 1, their_did, message, signature)

    # 8. Close wallets
    await wallet.close_wallet(their_wallet_handle)
    await wallet.close_wallet(my_wallet_handle)
