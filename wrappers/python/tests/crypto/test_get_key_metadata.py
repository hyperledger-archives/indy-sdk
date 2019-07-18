import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_get_key_metadata_works(wallet_handle, metadata):
    verkey = await did.create_key(wallet_handle, "{}")
    await did.set_key_metadata(wallet_handle, verkey, metadata)
    received_metadata = await did.get_key_metadata(wallet_handle, verkey)
    assert metadata == received_metadata


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_empty_string(wallet_handle):
    verkey = await did.create_key(wallet_handle, "{}")
    await did.set_key_metadata(wallet_handle, verkey, '')
    received_metadata = await did.get_key_metadata(wallet_handle, verkey)
    assert '' == received_metadata


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_no_key(wallet_handle, verkey_my1):
    with pytest.raises(error.WalletItemNotFound):
        await did.get_key_metadata(wallet_handle, verkey_my1)


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_no_metadata(wallet_handle):
    verkey = await did.create_key(wallet_handle, "{}")
    with pytest.raises(error.WalletItemNotFound):
        await did.get_key_metadata(wallet_handle, verkey)


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_invalid_handle(wallet_handle, metadata):
    verkey = await did.create_key(wallet_handle, "{}")
    await did.set_key_metadata(wallet_handle, verkey, metadata)
    with pytest.raises(error.WalletInvalidHandle):
        invalid_wallet_handle = wallet_handle + 1
        await did.get_key_metadata(invalid_wallet_handle, verkey)
