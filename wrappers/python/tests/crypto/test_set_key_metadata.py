import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_set_key_metadata_works(wallet_handle, metadata):
    verkey = await did.create_key(wallet_handle, "{}")
    await did.set_key_metadata(wallet_handle, verkey, metadata)


@pytest.mark.asyncio
async def test_set_key_metadata_works_for_empty_string(wallet_handle):
    verkey = await did.create_key(wallet_handle, "{}")
    await did.set_key_metadata(wallet_handle, verkey, '')


@pytest.mark.asyncio
async def test_set_key_metadata_works_for_invalid_key(wallet_handle, metadata):
    with pytest.raises(error.CommonInvalidStructure):
        await did.set_key_metadata(wallet_handle, 'invalid_base58string', metadata)


@pytest.mark.asyncio
async def test_set_key_metadata_works_for_invalid_handle(wallet_handle, metadata):
    verkey = await did.create_key(wallet_handle, "{}")
    with pytest.raises(error.WalletInvalidHandle):
        invalid_wallet_handle = wallet_handle + 1
        await did.set_key_metadata(invalid_wallet_handle, verkey, metadata)
