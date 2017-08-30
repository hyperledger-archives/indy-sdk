from indy import signus, wallet

import json
import logging

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)


async def demo():
    logger.info("Signus sample -> started")

    my_wallet_name = 'my_wallet'
    their_wallet_name = 'their_wallet'
    pool_name = 'pool1'
    seed_trustee1 = "000000000000000000000000Trustee1"

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

    #  9. Delete wallets
    await wallet.delete_wallet(my_wallet_name, None)
    await wallet.delete_wallet(their_wallet_name, None)

    logger.info("Signus sample -> completed")
