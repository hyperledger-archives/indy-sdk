import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_get_did_metadata_works(wallet_handle, metadata):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, metadata)
    received_metadata = await did.get_did_metadata(wallet_handle, _did)
    assert metadata == received_metadata


@pytest.mark.asyncio
async def test_get_did_metadata_works_for_empty_string(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, '')
    received_metadata = await did.get_did_metadata(wallet_handle, _did)
    assert '' == received_metadata


@pytest.mark.asyncio
async def test_get_did_metadata_works_for_no_metadata(wallet_handle, did_my1):
    with pytest.raises(error.WalletItemNotFound):
        await did.get_did_metadata(wallet_handle, did_my1)
