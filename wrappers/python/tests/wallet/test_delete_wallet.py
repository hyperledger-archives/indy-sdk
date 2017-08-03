from indy_sdk import IndyError
from indy_sdk import wallet
from indy_sdk.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_delete_wallet_works(cleanup_storage):
    await wallet.create_wallet('pool1', 'wallet_1', None, None, None)
    await wallet.delete_wallet('wallet_1', None)
    await wallet.create_wallet('pool1', 'wallet_1', None, None, None)


@pytest.mark.asyncio
async def test_delete_wallet_works_for_closed(cleanup_storage):
    await wallet.create_wallet('pool1', 'wallet_1', None, None, None)
    wallet_handle = await wallet.open_wallet('wallet_1', None, None)
    await wallet.close_wallet(wallet_handle)
    await wallet.delete_wallet('wallet_1', None)
    await wallet.create_wallet('pool1', 'wallet_1', None, None, None)


@pytest.mark.skip(reason="There is BUG in indy_sdk")
async def test_delete_wallet_works_for_opened(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet('pool1', 'wallet_for_opened', None, None, None)
        await wallet.open_wallet('wallet_for_opened', None, None)
        await wallet.delete_wallet('wallet_for_opened', None)
    assert ErrorCode.CommonIOError == e.value.error_code


@pytest.mark.asyncio
async def test_delete_wallet_works_for_twice(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet('pool1', 'wallet_for_twice', None, None, None)
        await wallet.delete_wallet('wallet_for_twice', None)
        await wallet.delete_wallet('wallet_for_twice', None)
    assert ErrorCode.CommonIOError == e.value.error_code


@pytest.mark.asyncio
async def test_delete_wallet_works_for_not_created(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet('wallet_not_created', None)
    assert ErrorCode.CommonIOError == e.value.error_code