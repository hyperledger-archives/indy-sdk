import pytest

from indy import IndyError
from indy import wallet
from indy.error import ErrorCode


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_config", [None, '{"freshness_time":1000}'])
async def test_open_wallet_works(wallet_config, wallet_handle):
    pass


@pytest.mark.asyncio
async def test_open_wallet_works_for_not_created_wallet(credentials):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet('wallet_not_created', None, credentials)
    assert ErrorCode.CommonIOError == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_twice(wallet_name, wallet_handle, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet(wallet_name, None, credentials)

    assert ErrorCode.WalletAlreadyOpenedError == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_missed_key(xwallet, wallet_name):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet(wallet_name, None, "{}")
    assert ErrorCode.WalletInputError == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.skip(reason="TODO: FIXME: Create a bug!!!")
async def test_open_wallet_works_for_changing_credentials(pool_name):
    await wallet.create_wallet(pool_name, 'works_for_changing_credentials', None, None, '{"key":"key"}')
    handle = await wallet.open_wallet('works_for_changing_credentials', None, '{"key":"key", "rekey":"other_key"}')
    await wallet.close_wallet(handle)

    handle = await wallet.open_wallet('works_for_changing_credentials', None, '{"key":"other_key"}')
    await wallet.close_wallet(handle)
