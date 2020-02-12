import pytest

from indy import wallet, error


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize(
    "wallet_config",
    ['{"id":"wallet1"}',
     '{"id":"wallet1", "storage_type":"default"}'])
async def test_create_wallet_works(wallet_config, xwallet):
    pass


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_create_wallet_works_for_unknown_type(credentials):
    wallet_config = '{"id":"wallet1", "storage_type":"unknown_type"}'
    with pytest.raises(error.WalletUnknownTypeError):
        await wallet.create_wallet(wallet_config, credentials)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_create_wallet_works_for_duplicate_name(xwallet, wallet_config, credentials):
    with pytest.raises(error.WalletAlreadyExistsError):
        await wallet.create_wallet(wallet_config, credentials)
