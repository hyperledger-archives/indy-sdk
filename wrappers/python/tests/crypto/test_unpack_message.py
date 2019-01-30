import json
import pytest

from indy import IndyError
from indy import crypto

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_pack_message_and_unpack_message_authcrypt_works(wallet_handle, identity_my1, identity_steward1,
                                                               pack_message):
    # setup keys
    _, sender_vk = identity_my1
    _, steward_vk = identity_steward1
    recipient_verkeys = [steward_vk]

    # run pack and unpack
    packed_message = await crypto.pack_message(wallet_handle, pack_message, recipient_verkeys, sender_vk)
    unpacked_message = await crypto.unpack_message(wallet_handle, packed_message)

    # test function
    unpacked_message_json = json.loads(unpacked_message.decode("utf-8"))
    assert unpacked_message_json['message'] == pack_message
    assert unpacked_message_json['recipient_verkey'] == steward_vk
    assert unpacked_message_json['sender_verkey'] == sender_vk


@pytest.mark.asyncio
async def test_pack_message_and_unpack_message_anoncrypt_works(wallet_handle, identity_steward1, pack_message):
    # setup keys
    _, steward_vk = identity_steward1
    recipient_verkeys = [steward_vk]

    # run pack and unpack
    packed_message = await crypto.pack_message(wallet_handle, pack_message, recipient_verkeys, None)
    unpacked_message = await crypto.unpack_message(wallet_handle, packed_message)

    # test function
    unpacked_message_json = json.loads(unpacked_message.decode("utf-8"))
    assert unpacked_message_json['message'] == pack_message
    assert unpacked_message_json['recipient_verkey'] == steward_vk
    assert 'sender_verkey' not in unpacked_message_json


@pytest.mark.asyncio
async def test_pack_message_and_unpack_message_missing_verkey(wallet_handle, identity_my1, verkey_my2, pack_message):
    # setup keys
    _, sender_vk = identity_my1
    recipient_verkeys = [verkey_my2]

    # run pack and unpack
    packed_message = await crypto.pack_message(wallet_handle, pack_message, recipient_verkeys, sender_vk)

    with pytest.raises(IndyError) as e:
        await crypto.unpack_message(wallet_handle, packed_message)
    assert ErrorCode.WalletItemNotFound == e.value.error_code
