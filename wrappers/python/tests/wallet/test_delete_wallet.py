import pytest

from indy import wallet, error


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
    with pytest.raises(error.CommonInvalidState):
        await wallet.delete_wallet(wallet_config, credentials)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("xwallet_cleanup", [False])
async def test_delete_wallet_works_for_twice(wallet_config, xwallet, credentials):
    await wallet.delete_wallet(wallet_config, credentials)

    with pytest.raises(error.WalletNotFoundError):
        await wallet.delete_wallet(wallet_config, credentials)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_delete_wallet_works_for_not_created(wallet_config, path_home, credentials):
    with pytest.raises(error.WalletNotFoundError):
        await wallet.delete_wallet(wallet_config, credentials)
