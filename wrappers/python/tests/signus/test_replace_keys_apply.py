from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_replace_keys_apply_works(wallet_handle):
    (did, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    await signus.replace_keys_start(wallet_handle, did, "{}")
    await signus.replace_keys_apply(wallet_handle, did)


@pytest.mark.asyncio
async def test_replace_keys_apply_works_without_calling_replace_start(wallet_handle):
    (did, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    with pytest.raises(IndyError) as e:
        await signus.replace_keys_apply(wallet_handle, did)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_apply_works_for_unknown_did(wallet_handle, did_my1):
    (did, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    await signus.replace_keys_start(wallet_handle, did, "{}")
    with pytest.raises(IndyError) as e:
        await signus.replace_keys_apply(wallet_handle, did_my1)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_apply_works_invalid_wallet_handle(wallet_handle):
    (did, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    await signus.replace_keys_start(wallet_handle, did, "{}")
    with pytest.raises(IndyError) as e:
        await signus.replace_keys_apply(wallet_handle + 1, did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
