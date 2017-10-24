from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_et_did_metadata_works(wallet_handle, did, metadata):
    await signus.set_did_metadata(wallet_handle, did, metadata)
    received_metadata = await signus.get_did_metadata(wallet_handle, did)
    assert metadata == received_metadata


@pytest.mark.asyncio
async def test_get_did_metadata_works_for_empty_string(wallet_handle, did):
    await signus.set_did_metadata(wallet_handle, did, '')
    received_metadata = await signus.get_did_metadata(wallet_handle, did)
    assert '' == received_metadata


@pytest.mark.asyncio
async def test_get_did_metadata_works_for_no_metadata(wallet_handle, did):
    with pytest.raises(IndyError) as e:
        await signus.get_did_metadata(wallet_handle, did)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_get_did_metadata_works_for_invalid_handle(wallet_handle, did, metadata):
    await signus.set_did_metadata(wallet_handle, did, metadata)
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await signus.get_did_metadata(invalid_wallet_handle, did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
