import json
import pytest
import asyncio

from indy import IndyError
from indy import crypto, did

from indy.error import ErrorCode

@pytest.mark.asyncio
async def test_pack_message_authcrypt_works(wallet_handle, seed_my1, verkey_my2, pack_message):
    sender_verkey = await did.create_key(wallet_handle, json.dumps({'seed': seed_my1}))
    receiver_verkeys = [verkey_my2]
    packed_message_bytes = await crypto.pack_message(wallet_handle, pack_message, receiver_verkeys, sender_verkey)
    packed_message_json = packed_message_bytes.decode("utf-8")
    json_message = json.loads(packed_message_json)
    assert json_message['protected'] != ""
    assert json_message['tag'] != ""
    assert json_message['ciphertext'] != ""
    assert json_message['iv'] != ""

@pytest.mark.asyncio
async def test_pack_message_anoncrypt_works(wallet_handle, verkey_my2):
    receiver_verkeys = [verkey_my2]
    packed_message_bytes = await crypto.pack_message(wallet_handle, "pack_message", receiver_verkeys, None)
    packed_message_json = packed_message_bytes.decode("utf-8")
    json_message = json.loads(packed_message_json)
    assert json_message['protected'] != ""
    assert json_message['tag'] != ""
    assert json_message['ciphertext'] != ""
    assert json_message['iv'] != ""

# @pytest.mark.asyncio
# async def test_pack_message_invalid_verkey(wallet_handle, seed_my1, verkey_my2):
#     sender_verkey = "INVALID_VERKEY"
#     receiver_verkeys = [verkey_my2]
#     packed_message = await crypto.pack_message(wallet_handle, "pack_message", receiver_verkeys, sender_verkey)
#     print(packed_message)