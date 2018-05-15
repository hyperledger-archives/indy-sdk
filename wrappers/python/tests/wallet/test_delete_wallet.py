import pytest

from indy import IndyError
from indy import wallet
from indy.error import ErrorCode


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup", [False])
async def test_delete_wallet_works(wallet_name, xwallet, credentials):
    await wallet.delete_wallet(wallet_name, credentials)


@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup, wallet_handle_cleanup", [(False, False)])
async def test_delete_wallet_works_for_closed(wallet_name, wallet_handle, credentials):
    await wallet.close_wallet(wallet_handle)
    await wallet.delete_wallet(wallet_name, credentials)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.skip(reason="TODO: FIXME: Create a bug!!!")
async def test_delete_wallet_works_for_opened(wallet_name, wallet_handle, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_name, credentials)
    assert ErrorCode.CommonIOError == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup", [False])
async def test_delete_wallet_works_for_twice(wallet_name, xwallet, credentials):
    await wallet.delete_wallet(wallet_name, credentials)

    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_name, credentials)

    assert ErrorCode.CommonIOError == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_delete_wallet_works_for_not_created(wallet_name, path_home, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_name, credentials)

    assert ErrorCode.CommonIOError == e.value.error_code