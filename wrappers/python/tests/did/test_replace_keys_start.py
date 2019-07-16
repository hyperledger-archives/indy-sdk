import json

import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_replace_keys_start_works(wallet_handle):
    (_did, ver_key) = await did.create_and_store_my_did(wallet_handle, "{}")
    new_ver_key = await did.replace_keys_start(wallet_handle, _did, "{}")
    assert new_ver_key != _did


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_seed(wallet_handle, seed_my1, verkey_my1):
    (_did, ver_key) = await did.create_and_store_my_did(wallet_handle, "{}")
    new_ver_key = await did.replace_keys_start(wallet_handle, _did, json.dumps({'seed': seed_my1}))
    assert new_ver_key != _did
    assert verkey_my1 == new_ver_key


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_correct_crypto_type(wallet_handle, crypto_type):
    (_did, ver_key) = await did.create_and_store_my_did(wallet_handle, "{}")
    new_ver_key = await did.replace_keys_start(wallet_handle, _did, json.dumps({"crypto_type": crypto_type}))
    assert new_ver_key != _did


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_not_exists_did(wallet_handle, did_my1):
    with pytest.raises(error.WalletItemNotFound):
        await did.replace_keys_start(wallet_handle, did_my1, "{}")


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_invalid_handle(wallet_handle):
    with pytest.raises(error.WalletInvalidHandle):
        (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
        await did.replace_keys_start(wallet_handle + 1, _did, "{}")


@pytest.mark.asyncio
async def test_replace_keys_works_start_for_invalid_crypto_type(wallet_handle):
    with pytest.raises(error.UnknownCryptoTypeError):
        (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
        await did.replace_keys_start(wallet_handle, _did, '{"crypto_type": "type"}')
