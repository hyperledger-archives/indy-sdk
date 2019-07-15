import pytest

from indy import wallet, error


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_handle_cleanup", [False])
async def test_close_wallet_works(wallet_config, wallet_handle, credentials):
    await wallet.close_wallet(wallet_handle)

    wallet_handle = await wallet.open_wallet(wallet_config, credentials)
    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_handle_cleanup", [False])
async def test_close_wallet_works_for_twice(wallet_handle):
    with pytest.raises(error.WalletInvalidHandle):
        await wallet.close_wallet(wallet_handle)
        await wallet.close_wallet(wallet_handle)
