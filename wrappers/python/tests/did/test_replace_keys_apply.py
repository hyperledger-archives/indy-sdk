from indy import IndyError
from indy import did
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_replace_keys_apply_works(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.replace_keys_start(wallet_handle, _did, "{}")
    await did.replace_keys_apply(wallet_handle, _did)


@pytest.mark.asyncio
async def test_replace_keys_apply_works_without_calling_replace_start(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    with pytest.raises(IndyError) as e:
        await did.replace_keys_apply(wallet_handle, _did)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_apply_works_for_unknown_did(wallet_handle, did_my1):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.replace_keys_start(wallet_handle, _did, "{}")
    with pytest.raises(IndyError) as e:
        await did.replace_keys_apply(wallet_handle, did_my1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_replace_keys_apply_works_invalid_wallet_handle(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.replace_keys_start(wallet_handle, _did, "{}")
    with pytest.raises(IndyError) as e:
        await did.replace_keys_apply(wallet_handle + 1, _did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
