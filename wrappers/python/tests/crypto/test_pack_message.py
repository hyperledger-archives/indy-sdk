import json
import pytest

from indy import crypto, error


@pytest.mark.asyncio
async def test_pack_message_authcrypt_works(wallet_handle, identity_my1, verkey_my2, pack_message):
    _, sender_verkey = identity_my1
    receiver_verkeys = [verkey_my2]
    packed_message_bytes = await crypto.pack_message(wallet_handle, pack_message, receiver_verkeys, sender_verkey)
    packed_message_json = packed_message_bytes.decode("utf-8")
    json_message = json.loads(packed_message_json)
    assert json_message['protected']
    assert json_message['tag']
    assert json_message['ciphertext']
    assert json_message['iv']


@pytest.mark.asyncio
async def test_pack_message_anoncrypt_works(wallet_handle, verkey_my2, pack_message):
    receiver_verkeys = [verkey_my2]
    packed_message_bytes = await crypto.pack_message(wallet_handle, pack_message, receiver_verkeys, None)
    packed_message_json = packed_message_bytes.decode("utf-8")
    json_message = json.loads(packed_message_json)
    assert json_message['protected']
    assert json_message['tag']
    assert json_message['ciphertext']
    assert json_message['iv']


@pytest.mark.asyncio
async def test_pack_message_invalid_sender_verkey(wallet_handle, verkey_my2, pack_message):
    receiver_verkeys = [verkey_my2]
    with pytest.raises(error.CommonInvalidStructure):
        await crypto.pack_message(wallet_handle, pack_message, receiver_verkeys, "INVALID_SENDER_VERKEY")


@pytest.mark.asyncio
async def test_pack_message_invalid_receiver_verkey(wallet_handle, verkey_my2, identity_my1, pack_message):
    _, sender_verkey = identity_my1
    # This test is expected to fail because the type is not a list
    receiver_verkeys = verkey_my2
    with pytest.raises(error.CommonInvalidParam4):
        await crypto.pack_message(wallet_handle, pack_message, receiver_verkeys, sender_verkey)


@pytest.mark.asyncio
async def test_pack_message_invalid_wallet_handle(wallet_handle, verkey_my2, identity_my1, pack_message):
    sender_verkey, _ = identity_my1
    receiver_verkeys = [verkey_my2]
    with pytest.raises(error.WalletInvalidHandle):
        await crypto.pack_message(wallet_handle + 1, pack_message, receiver_verkeys, sender_verkey)
