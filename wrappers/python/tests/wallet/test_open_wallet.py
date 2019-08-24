import pytest

from indy import wallet, error


@pytest.mark.asyncio
async def test_open_wallet_works(wallet_handle):
    pass


@pytest.mark.asyncio
async def test_open_wallet_works_for_not_created_wallet(wallet_config, credentials):
    with pytest.raises(error.WalletNotFoundError):
        await wallet.open_wallet(wallet_config, credentials)


@pytest.mark.asyncio
async def test_open_wallet_works_for_twice(wallet_handle, wallet_config, credentials):
    with pytest.raises(error.WalletAlreadyOpenedError):
        await wallet.open_wallet(wallet_config, credentials)


@pytest.mark.asyncio
async def test_open_wallet_works_for_missed_key(xwallet, wallet_config):
    with pytest.raises(error.CommonInvalidStructure):
        await wallet.open_wallet(wallet_config, "{}")
