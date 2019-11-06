import pytest

from indy import did


@pytest.mark.asyncio
async def test_set_did_metadata_works(wallet_handle, metadata):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, metadata)


@pytest.mark.asyncio
async def test_set_did_metadata_works_for_empty_string(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, '')
