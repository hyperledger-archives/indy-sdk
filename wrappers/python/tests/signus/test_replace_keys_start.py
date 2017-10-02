from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_replace_keys_start_works(wallet_handle):
    (did, ver_key, pk) = await signus.create_and_store_my_did(wallet_handle, "{}")
    (new_ver_key, new_pk) = await signus.replace_keys_start(wallet_handle, did, "{}")
    assert (new_ver_key != did) and (new_pk != pk)


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_seed(wallet_handle):
    (did, ver_key, pk) = await signus.create_and_store_my_did(wallet_handle, "{}")
    (new_ver_key, new_pk) = await signus.replace_keys_start(wallet_handle, did, '{"seed": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}')
    assert (new_ver_key != did) and (new_pk != pk)
    assert 'CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW' == new_ver_key


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_correct_crypto_type(wallet_handle):
    (did, ver_key, pk) = await signus.create_and_store_my_did(wallet_handle, "{}")
    (new_ver_key, new_pk) = await signus.replace_keys_start(wallet_handle, did, '{"crypto_type": "ed25519"}')
    assert (new_ver_key != did) and (new_pk != pk)


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_not_exists_did(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.replace_keys_start(wallet_handle, 'unknowndid', "{}")
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_start_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
        await signus.replace_keys_start(wallet_handle + 1, did, "{}")
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_works_start_for_invalid_crypto_type(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
        await signus.replace_keys_start(wallet_handle, did, '{"crypto_type": "type"}')
    assert ErrorCode.SignusUnknownCryptoError == e.value.error_code
