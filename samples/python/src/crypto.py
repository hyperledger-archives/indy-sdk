import time

from indy import did, crypto, wallet

import json
import logging

from indy import pool

from src.utils import run_coroutine, PROTOCOL_VERSION

logger = logging.getLogger(__name__)


async def demo():
    logger.info("Crypto sample -> started")

    signer = {
        'wallet_config': json.dumps({'id': 'signer_wallet'}),
        'wallet_credentials': json.dumps({'key': 'signer_wallet_key'})
    }
    verifier = {
        'wallet_config': json.dumps({"id": "verifier_wallet"}),
        'wallet_credentials': json.dumps({"key": "verifier_wallet_key"})
    }

    # Set protocol version 2 to work with Indy Node 1.4
    await pool.set_protocol_version(PROTOCOL_VERSION)

    # 1. Create Wallet and Get Wallet Handle
    await wallet.create_wallet(signer['wallet_config'], signer['wallet_credentials'])
    signer['wallet'] = await wallet.open_wallet(signer['wallet_config'], signer['wallet_credentials'])

    await wallet.create_wallet(verifier['wallet_config'], verifier['wallet_credentials'])
    verifier['wallet'] = await wallet.open_wallet(verifier['wallet_config'], verifier['wallet_credentials'])

    # 2. Signer Create DID
    (signer['did'], signer['verkey']) = await did.create_and_store_my_did(signer['wallet'], "{}")

    # 3. Verifier Create DID
    (verifier['did'], verifier['verkey']) = await did.create_and_store_my_did(verifier['wallet'], "{}")

    signer['verifier_did'] = verifier['did']
    signer['verifier_verkey'] = verifier['verkey']
    verifier['signer_did'] = signer['did']
    verifier['signer_verkey'] = signer['verkey']

    # 4. Signer auth crypt message
    message = json.dumps({
        "reqId": 1495034346617224651,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
        }
    })

    signer['encrypted_message'] = \
        await crypto.auth_crypt(signer['wallet'], signer['verkey'], signer['verifier_verkey'], message.encode('utf-8'))
    verifier['encrypted_message'] = signer['encrypted_message']

    # 5. Verifier decrypt message
    verkey, decrypted_message = \
        await crypto.auth_decrypt(verifier['wallet'], verifier['verkey'], verifier['encrypted_message'])
    assert verifier['signer_verkey'] == verkey
    assert message == decrypted_message.decode("utf-8")

    # 6. Close and delete Signer wallet
    await wallet.close_wallet(signer['wallet'])
    await wallet.delete_wallet(signer['wallet_config'], signer['wallet_credentials'])

    # 7. Close and delete Verifier wallet
    await wallet.close_wallet(verifier['wallet'])
    await wallet.delete_wallet(verifier['wallet_config'], verifier['wallet_credentials'])

    logger.info("Crypto sample -> completed")


if __name__ == '__main__':
    run_coroutine(demo)
    time.sleep(1)  # FIXME waiting for libindy thread complete
