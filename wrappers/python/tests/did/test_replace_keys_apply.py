import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_replace_keys_apply_works(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.replace_keys_start(wallet_handle, _did, "{}")
    await did.replace_keys_apply(wallet_handle, _did)


@pytest.mark.asyncio
async def test_replace_keys_apply_works_without_calling_replace_start(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    with pytest.raises(error.WalletItemNotFound):
        await did.replace_keys_apply(wallet_handle, _did)


@pytest.mark.asyncio
async def test_replace_keys_apply_works_for_unknown_did(wallet_handle, did_my1):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.replace_keys_start(wallet_handle, _did, "{}")
    with pytest.raises(error.WalletItemNotFound):
        await did.replace_keys_apply(wallet_handle, did_my1)
