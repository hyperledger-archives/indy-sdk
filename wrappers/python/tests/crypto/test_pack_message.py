import json

from indy import IndyError
from indy import crypto, did

import pytest

from indy.error import ErrorCode

@pytest.mark.asyncio
async def test_pack_message_authcrypt_works(wallet_handle, seed_my1, verkey_my2):
    sender_verkey = await did.create_key(wallet_handle, json.dumps({'seed': seed_my1}))
    receiver_verkeys = [verkey_my2]
    packed_message = await crypto.pack_message(wallet_handle, "Hello World", receiver_verkeys, sender_verkey)
    print(packed_message)

