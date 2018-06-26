import time

from indy import did, crypto, wallet

import json
import logging

from indy import pool

from src.utils import run_coroutine, PROTOCOL_VERSION

logger = logging.getLogger(__name__)


async def demo():
    logger.info("Crypto sample -> started")

    wallet_name = 'wallet1'
    pool_name = 'pool1'

    # Set protocol version 2 to work with Indy Node 1.4
    await pool.set_protocol_version(PROTOCOL_VERSION)

    # 1. Create Wallet and Get Wallet Handle
    wallet_credentials = json.dumps({"key": "wallet_key"})
    await wallet.create_wallet(pool_name, wallet_name, None, None, wallet_credentials)
    wallet_handle = await wallet.open_wallet(wallet_name, None, wallet_credentials)

    # 2. Create DID
    (_, their_verkey) = await did.create_and_store_my_did(wallet_handle, "{}")

    # 3. Sign message
    message = json.dumps({
        "reqId": 1495034346617224651,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
        }
    })

    signature = await crypto.crypto_sign(wallet_handle, their_verkey, message)

    # 4. verify message
    assert await crypto.crypto_verify(their_verkey, message, signature)

    # 5. Close wallets
    await wallet.close_wallet(wallet_handle)

    # 6. Delete wallets
    await wallet.delete_wallet(wallet_name, wallet_credentials)

    logger.info("Crypto sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
