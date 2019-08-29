import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_get_key_metadata_works(wallet_handle, metadata):
    verkey = await did.create_key(wallet_handle, "{}")
    await did.set_key_metadata(wallet_handle, verkey, metadata)
    received_metadata = await did.get_key_metadata(wallet_handle, verkey)
    assert metadata == received_metadata


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_no_metadata(wallet_handle):
    verkey = await did.create_key(wallet_handle, "{}")
    with pytest.raises(error.WalletItemNotFound):
        await did.get_key_metadata(wallet_handle, verkey)
