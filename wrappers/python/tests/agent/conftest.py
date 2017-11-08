import json
import base64

import pytest

from indy import signus, wallet

from tests.conftest import seed_my2


@pytest.mark.asyncio
async def check_message(encrypted_msg, message=None, sender_verkey=None):
    await wallet.create_wallet('pool', 'local_wallet', None, None, None)
    local_wallet_handle = await wallet.open_wallet('local_wallet', None, None)

    (recipient_did, _) = await signus.create_and_store_my_did(local_wallet_handle, json.dumps({'seed': seed_my2()}))

    decrypted_message = await signus.decrypt_sealed(local_wallet_handle, recipient_did, encrypted_msg)
    decrypted_msg = json.loads(decrypted_message.decode("utf-8"))

    if decrypted_msg['auth']:
        assert sender_verkey == decrypted_msg['sender']
        assert decrypted_msg['nonce']
        assert decrypted_msg['msg']
    else:
        assert message == base64.b64decode(decrypted_msg['msg'])

    await wallet.close_wallet(local_wallet_handle)
    await wallet.delete_wallet('local_wallet', None)
