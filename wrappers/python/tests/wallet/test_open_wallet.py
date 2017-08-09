import pytest

from indy import IndyError
from indy import wallet
from indy.error import ErrorCode


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_config", [None, '{"freshness_time":1000}'])
async def test_open_wallet_works(wallet_config, wallet_handle):
    pass


@pytest.mark.asyncio
async def test_open_wallet_works_for_not_created_wallet():
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet('wallet_not_created', None, None)
    assert ErrorCode.CommonIOError == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_twice(wallet_name, wallet_handle):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet(wallet_name, None, None)

    assert ErrorCode.WalletAlreadyOpenedError == e.value.error_code
