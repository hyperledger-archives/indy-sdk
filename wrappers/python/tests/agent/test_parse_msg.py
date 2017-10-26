import json

from indy import IndyError
from indy import agent, signus

import pytest

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_parse_msg_works_for_authenticated_message(wallet_handle, identity_steward1, identity_trustee1, message):
    (_, sender_verkey) = identity_steward1
    (_, recipient_verkey) = identity_trustee1
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, recipient_verkey, message)
    verkey, parsed_message = await agent.parse_msg(wallet_handle, recipient_verkey, encrypted_msg)
    assert sender_verkey == verkey
    assert message == parsed_message


@pytest.mark.asyncio
async def test_parse_msg_works_for_anonymous_message(wallet_handle, identity_trustee1, message):
    (_, recipient_verkey) = identity_trustee1
    encrypted_msg = await agent.prep_anonymous_msg(recipient_verkey, message)
    verkey, parsed_message = await agent.parse_msg(wallet_handle, recipient_verkey, encrypted_msg)
    assert not verkey
    assert message == parsed_message


@pytest.mark.asyncio
async def test_parse_msg_works_for_invalid_authenticated_msg(pool_handle, wallet_handle, identity_steward1,
                                                             identity_trustee1):
    (_, sender_verkey) = identity_steward1
    (recipient_did, recipient_verkey) = identity_trustee1
    await signus.store_their_did(wallet_handle, json.dumps({'did': recipient_did, 'verkey': recipient_verkey}))

    msg = json.dumps({
        'auth': True,
        'nonce': [1, 2, 3, 4, 5, 6],
        'sender': sender_verkey,
        'msg': 'unencrypted message'
    })
    encrypted_msg = await signus.encrypt_sealed(wallet_handle, pool_handle, recipient_did, msg.encode('utf-8'))
    with pytest.raises(IndyError) as e:
        await agent.parse_msg(wallet_handle, recipient_verkey, encrypted_msg)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_parse_msg_works_for_invalid_anonymous_msg(wallet_handle, identity_trustee1):
    (_, recipient_verkey) = identity_trustee1
    msg = "unencrypted message"
    with pytest.raises(IndyError) as e:
        await agent.parse_msg(wallet_handle, recipient_verkey, msg.encode('utf-8'))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_parse_msg_msg_works_for_unknown_recipient_vk(wallet_handle, verkey_my1, message):
    encrypted_msg = await agent.prep_anonymous_msg(verkey_my1, message)
    with pytest.raises(IndyError) as e:
        await agent.parse_msg(wallet_handle, verkey_my1, encrypted_msg)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_parse_msg_msg_works_for_invalid_handle(wallet_handle, identity_trustee1, message):
    (_, recipient_verkey) = identity_trustee1
    encrypted_msg = await agent.prep_anonymous_msg(recipient_verkey, message)
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await agent.parse_msg(invalid_wallet_handle, recipient_verkey, encrypted_msg)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
