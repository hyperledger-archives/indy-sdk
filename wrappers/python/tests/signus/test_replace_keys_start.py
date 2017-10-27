import json

from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_replace_keys_start_works(wallet_handle):
    (did, ver_key) = await signus.create_and_store_my_did(wallet_handle, "{}")
    new_ver_key = await signus.replace_keys_start(wallet_handle, did, "{}")
    assert new_ver_key != did


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_seed(wallet_handle, seed_my1, verkey_my1):
    (did, ver_key) = await signus.create_and_store_my_did(wallet_handle, "{}")
    new_ver_key = await signus.replace_keys_start(wallet_handle, did, json.dumps({'seed': seed_my1}))
    assert new_ver_key != did
    assert verkey_my1 == new_ver_key


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_correct_crypto_type(wallet_handle, crypto_type):
    (did, ver_key) = await signus.create_and_store_my_did(wallet_handle, "{}")
    new_ver_key = await signus.replace_keys_start(wallet_handle, did, json.dumps({"crypto_type": crypto_type}))
    assert new_ver_key != did


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_not_exists_did(wallet_handle, did_my1):
    with pytest.raises(IndyError) as e:
        await signus.replace_keys_start(wallet_handle, did_my1, "{}")
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
        await signus.replace_keys_start(wallet_handle + 1, did, "{}")
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_works_start_for_invalid_crypto_type(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
        await signus.replace_keys_start(wallet_handle, did, '{"crypto_type": "type"}')
    assert ErrorCode.SignusUnknownCryptoError == e.value.error_code
