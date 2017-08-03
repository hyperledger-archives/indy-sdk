from indy_sdk import IndyError
from indy_sdk import wallet
from indy_sdk.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_close_wallet_works(cleanup_storage):
    await wallet.create_wallet('pool1', 'wallet_1', None, None, None)

    wallet_handle = await wallet.open_wallet('wallet_1', None, None)
    await wallet.close_wallet(wallet_handle)

    wallet_handle = await wallet.open_wallet('wallet_1', None, None)
    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
async def test_close_wallet_works_for_twice(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet('pool1', 'wallet_for_twice', None, None, None)

        wallet_handle = await wallet.open_wallet('wallet_for_twice', None, None)
        await wallet.close_wallet(wallet_handle)
        await wallet.close_wallet(wallet_handle)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
