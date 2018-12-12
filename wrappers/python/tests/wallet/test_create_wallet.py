import pytest

from indy import wallet
from indy.error import IndyError, ErrorCode


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
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet(wallet_config, credentials)
    assert ErrorCode.WalletUnknownTypeError == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_create_wallet_works_for_duplicate_name(xwallet, wallet_config, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet(wallet_config, credentials)

    assert ErrorCode.WalletAlreadyExistsError == e.value.error_code
