from indy import IndyError
from indy import did
from indy.error import ErrorCode

import pytest


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
    with pytest.raises(IndyError) as e:
        await did.get_did_metadata(wallet_handle, did_my1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_get_did_metadata_works_for_invalid_handle(wallet_handle, metadata):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, "{}")
    await did.set_did_metadata(wallet_handle, _did, metadata)
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await did.get_did_metadata(invalid_wallet_handle, _did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_get_did_metadata_works_for_not_found_did(wallet_handle, did_my1):
    with pytest.raises(IndyError) as e:
        await did.get_did_metadata(wallet_handle, did_my1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code
