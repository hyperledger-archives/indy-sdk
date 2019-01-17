import pytest

from indy import IndyError
from indy import wallet
from indy.error import ErrorCode


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup", [False])
async def test_delete_wallet_works(wallet_config, xwallet, credentials):
    await wallet.delete_wallet(wallet_config, credentials)


@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup, wallet_handle_cleanup", [(False, False)])
async def test_delete_wallet_works_for_closed(wallet_config, wallet_handle, credentials):
    await wallet.close_wallet(wallet_handle)
    await wallet.delete_wallet(wallet_config, credentials)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_delete_wallet_works_for_opened(wallet_config, wallet_handle, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_config, credentials)
    assert ErrorCode.CommonInvalidState == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup", [False])
async def test_delete_wallet_works_for_twice(wallet_config, xwallet, credentials):
    await wallet.delete_wallet(wallet_config, credentials)

    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_config, credentials)

    assert ErrorCode.WalletNotFoundError == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_delete_wallet_works_for_not_created(wallet_config, path_home, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.delete_wallet(wallet_config, credentials)

    assert ErrorCode.WalletNotFoundError == e.value.error_code