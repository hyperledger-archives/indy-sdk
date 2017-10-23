import json

from indy import IndyError
from indy import agent, signus

import pytest

from indy.error import ErrorCode

message = '{"reqId":1496822211362017764}'.encode('utf-8')


@pytest.mark.asyncio
async def check_message(wallet_handle, recipient_did, sender_verkey, encrypted_msg):
    decrypted_message = await signus.decrypt_sealed(wallet_handle, recipient_did, encrypted_msg)
    decrypted_msg = json.loads(decrypted_message.decode("utf-8"))

    assert decrypted_msg['auth']
    assert sender_verkey == decrypted_msg['sender']
    assert decrypted_msg['nonce']
    assert decrypted_msg['msg']


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_key(wallet_handle, identity_steward1, seed_my1):
    (recipient_did, recipient_verkey) = identity_steward1
    (_, sender_verkey, _) = await signus.create_key(wallet_handle, json.dumps({'seed': seed_my1}))
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, recipient_verkey, message)
    check_message(wallet_handle, recipient_did, sender_verkey, encrypted_msg)


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_did(wallet_handle, identity_steward1, seed_my1):
    (recipient_did, recipient_verkey) = identity_steward1
    (_, sender_verkey, _) = await signus.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1}))
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, recipient_verkey, message)
    check_message(wallet_handle, recipient_did, sender_verkey, encrypted_msg)


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_did_as_cid(wallet_handle, identity_steward1, seed_my1):
    (recipient_did, recipient_verkey) = identity_steward1
    (_, sender_verkey, _) = await signus.create_and_store_my_did(wallet_handle,
                                                                 json.dumps({'seed': seed_my1, 'cid': True}))
    encrypted_msg = await agent.prep_msg(wallet_handle, sender_verkey, recipient_verkey, message)
    check_message(wallet_handle, recipient_did, sender_verkey, encrypted_msg)


@pytest.mark.asyncio
async def test_prep_msg_works_for_unknown_sender_verkey(wallet_handle, verkey_my1, verkey_my2):
    with pytest.raises(IndyError) as e:
        await agent.prep_msg(wallet_handle, verkey_my1, verkey_my2, message)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_prep_msg_works_for_invalid_handle(wallet_handle, verkey_my1, verkey_my2):
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await agent.prep_msg(invalid_wallet_handle, verkey_my1, verkey_my2, message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_prep_msg_works_for_invalid_recipient_vk(wallet_handle, verkey_my1):
    with pytest.raises(IndyError) as e:
        await agent.prep_msg(wallet_handle, verkey_my1, 'invalidVerkeyLength', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code

    with pytest.raises(IndyError) as e:
        await agent.prep_msg(wallet_handle, verkey_my1, 'CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
