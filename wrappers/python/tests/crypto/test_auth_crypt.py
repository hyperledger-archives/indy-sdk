import json

from indy import IndyError
from indy import crypto, did

import pytest

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_auth_crypt_works_for_created_key(wallet_handle, seed_my1, verkey_my2, message):
    verkey = await did.create_key(wallet_handle, json.dumps({'seed': seed_my1}))
    await crypto.auth_crypt(wallet_handle, verkey, verkey_my2, message)


@pytest.mark.asyncio
async def test_auth_crypt_works_for_created_did(wallet_handle, seed_my1, verkey_my2, message):
    (_, verkey) = await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1}))
    await crypto.auth_crypt(wallet_handle, verkey, verkey_my2, message)


@pytest.mark.asyncio
async def test_auth_crypt_works_for_created_did_as_cid(wallet_handle, seed_my1, verkey_my2, message):
    (_, verkey) = await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1, 'cid': True}))
    await crypto.auth_crypt(wallet_handle, verkey, verkey_my2, message)


@pytest.mark.asyncio
async def test_auth_crypt_works_for_unknown_sender_verkey(wallet_handle, verkey_my1, verkey_my2, message):
    with pytest.raises(IndyError) as e:
        await crypto.auth_crypt(wallet_handle, verkey_my1, verkey_my2, message)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_auth_crypt_works_for_invalid_handle(wallet_handle, verkey_my1, verkey_my2, message):
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await crypto.auth_crypt(invalid_wallet_handle, verkey_my1, verkey_my2, message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_auth_crypt_works_for_invalid_recipient_vk(wallet_handle, identity_trustee1, message):
    (_, key) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await crypto.auth_crypt(wallet_handle, key, 'CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
