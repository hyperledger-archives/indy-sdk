import json

import pytest

from indy import crypto, did, error


@pytest.mark.asyncio
async def test_auth_crypt_works_for_created_key(wallet_handle, seed_my1, verkey_my2, message):
    verkey = await did.create_key(wallet_handle, json.dumps({'seed': seed_my1}))
    await crypto.auth_crypt(wallet_handle, verkey, verkey_my2, message)


@pytest.mark.asyncio
async def test_auth_crypt_works_for_unknown_sender_verkey(wallet_handle, verkey_my1, verkey_my2, message):
    with pytest.raises(error.WalletItemNotFound):
        await crypto.auth_crypt(wallet_handle, verkey_my1, verkey_my2, message)


@pytest.mark.asyncio
async def test_auth_crypt_works_for_invalid_handle(wallet_handle, verkey_my1, verkey_my2, message):
    with pytest.raises(error.WalletInvalidHandle):
        invalid_wallet_handle = wallet_handle + 1
        await crypto.auth_crypt(invalid_wallet_handle, verkey_my1, verkey_my2, message)


@pytest.mark.asyncio
async def test_auth_crypt_works_for_invalid_recipient_vk(wallet_handle, identity_trustee1, message):
    (_, key) = identity_trustee1
    with pytest.raises(error.CommonInvalidStructure):
        await crypto.auth_crypt(wallet_handle, key, 'CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
