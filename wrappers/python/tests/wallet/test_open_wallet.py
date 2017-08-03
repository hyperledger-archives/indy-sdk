from indysdk import IndyError
from indysdk import wallet
from indysdk.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_open_wallet_works(cleanup_storage):
    await wallet.create_wallet('pool1', 'wallet1', None, None, None)
    wallet_handle = await wallet.open_wallet('wallet1', None, None)
    assert wallet_handle is not None

    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
async def test_open_wallet_works_for_config(cleanup_storage):
    await wallet.create_wallet('pool1', 'wallet1', None, None, None)
    wallet_handle = await wallet.open_wallet('wallet1', '{"freshness_time":1000}', None)
    assert wallet_handle is not None

    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
async def test_open_wallet_works_for_not_created_wallet(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet('wallet_not_created', None, None)
    assert ErrorCode.CommonIOError == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_twice(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet('pool1', 'wallet_twice', None, None, None)
        await wallet.open_wallet('wallet_twice', None, None)
        await wallet.open_wallet('wallet_twice', None, None)
    assert ErrorCode.WalletAlreadyOpenedError == e.value.error_code