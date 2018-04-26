from indy import IndyError
from indy import did
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_get_key_metadata_works(wallet_handle, verkey_my1, metadata):
    await did.set_key_metadata(wallet_handle, verkey_my1, metadata)
    received_metadata = await did.get_key_metadata(wallet_handle, verkey_my1)
    assert metadata == received_metadata


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_empty_string(wallet_handle, verkey_my1):
    await did.set_key_metadata(wallet_handle, verkey_my1, '')
    received_metadata = await did.get_key_metadata(wallet_handle, verkey_my1)
    assert '' == received_metadata


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_no_metadata(wallet_handle, verkey_my1):
    with pytest.raises(IndyError) as e:
        await did.get_key_metadata(wallet_handle, verkey_my1)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_get_key_metadata_works_for_invalid_handle(wallet_handle, verkey_my1, metadata):
    await did.set_key_metadata(wallet_handle, verkey_my1, metadata)
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await did.get_key_metadata(invalid_wallet_handle, verkey_my1)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
