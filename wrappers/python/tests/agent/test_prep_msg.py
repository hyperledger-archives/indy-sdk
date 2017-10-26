import json

from indy import IndyError
from indy import agent, signus

import pytest

from indy.error import ErrorCode
from tests.agent.conftest import check_message


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_key(wallet_handle, seed_my1, verkey_my2, message):
    sender_verkey = await signus.create_key(wallet_handle, json.dumps({'seed': seed_my1}))
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, verkey_my2, message)
    await check_message(encrypted_msg, None, sender_verkey)


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_did(wallet_handle, seed_my1, verkey_my2, message):
    (_, sender_verkey) = await signus.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1}))
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, verkey_my2, message)
    await check_message(encrypted_msg, None, sender_verkey)


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_did_as_cid(wallet_handle, seed_my1, verkey_my2, message):
    (_, sender_verkey) = await signus.create_and_store_my_did(wallet_handle,
                                                                 json.dumps({'seed': seed_my1, 'cid': True}))
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, verkey_my2, message)
    await check_message(encrypted_msg, None, sender_verkey)


@pytest.mark.asyncio
async def test_prep_msg_works_for_unknown_sender_verkey(wallet_handle, verkey_my1, verkey_my2, message):
    with pytest.raises(IndyError) as e:
        await agent.prep_msg(wallet_handle, verkey_my1, verkey_my2, message)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_prep_msg_works_for_invalid_handle(wallet_handle, verkey_my1, verkey_my2, message):
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await agent.prep_msg(invalid_wallet_handle, verkey_my1, verkey_my2, message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_prep_msg_works_for_invalid_recipient_vk(wallet_handle, identity_trustee1, message):
    (_, key) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await agent.prep_msg(wallet_handle, key, 'CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
