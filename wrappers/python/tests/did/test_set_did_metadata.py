from indy import IndyError
from indy import did
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_set_did_metadata_works(wallet_handle, metadata):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, metadata)


@pytest.mark.asyncio
async def test_set_did_metadata_works_for_replace(wallet_handle, metadata):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, metadata)
    received_metadata = await did.get_did_metadata(wallet_handle, _did)
    assert metadata == received_metadata

    new_metadata = 'new metadata'
    await did.set_did_metadata(wallet_handle, _did, new_metadata)
    updated_metadata = await did.get_did_metadata(wallet_handle, _did)
    assert new_metadata == updated_metadata


@pytest.mark.asyncio
async def test_set_did_metadata_works_for_empty_string(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, '')


@pytest.mark.asyncio
async def test_set_did_metadata_works_for_invalid_handle(wallet_handle, did_my1, metadata):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await did.set_did_metadata(invalid_wallet_handle, did_my1, metadata)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_set_did_metadata_works_for_unknown_did(wallet_handle, did_my1, metadata):
    await did.set_did_metadata(wallet_handle, did_my1, metadata)
