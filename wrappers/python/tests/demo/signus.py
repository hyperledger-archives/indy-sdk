from indy import signus, wallet

from tests.utils import storage

import pytest
import logging
import json


logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_signus_demo_works(cleanup_storage):
    pool_name = "create"
    my_wallet_name = "my_wallet"
    their_wallet_name = "their_wallet"

    # 1. Create My Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, my_wallet_name, None, None, None)
    my_wallet_handle = await wallet.open_wallet(my_wallet_name, None, None)

    # 2. Create Their Wallet and Get Wallet Handle
    await wallet.create_wallet(pool_name, their_wallet_name, None, None, None)
    their_wallet_handle = await wallet.open_wallet(their_wallet_name, None, None)

    # 3. Create My DID
    await signus.create_and_store_my_did(my_wallet_handle, "{}")

    # 4. Create Their DID from Trustee1 seed
    (their_did, their_verkey, their_pk) = \
        await signus.create_and_store_my_did(their_wallet_handle, '{"seed":"000000000000000000000000Trustee1"}')

    # 5. Store Their DID
    their_identity = {
        'did': their_did,
        'pk': their_pk,
        'verkey': their_verkey
    }

    await signus.store_their_did(my_wallet_handle, json.dumps(their_identity))

    # 6. Their sign message
    message = {
        "reqId": 1495034346617224651,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
        }
    }

    signed_msg = await signus.sign(their_wallet_handle, their_did, json.dumps(message))

    # 7. Their sign message
    assert await signus.verify_signature(my_wallet_handle, 1, their_did, signed_msg)

    # 8. Close wallets
    await wallet.close_wallet(their_wallet_handle)
    await wallet.close_wallet(my_wallet_handle)

