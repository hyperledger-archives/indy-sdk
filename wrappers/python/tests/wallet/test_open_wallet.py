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


@pytest.mark.asyncio
@pytest.mark.parametrize("credentials", [None, '{"key":"testkey"}'])
async def test_open_wallet_works_for_encrypted_wallet(wallet_handle, credentials):
    pass


@pytest.mark.asyncio
async def test_open_wallet_works_for_encrypted_wallet_with_invalid_credentials(xwallet, wallet_name):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet(wallet_name, None, '{"key":"otherkey"}')
    assert ErrorCode.WalletAccessFailed == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_changing_credentials(pool_name):
    await wallet.create_wallet(pool_name, 'works_for_changing_credentials', None, None, '{"key":"testkey"}')
    handle = await wallet.open_wallet('works_for_changing_credentials', None, '{"key":"testkey", "rekey":"newkey"}')
    await wallet.close_wallet(handle)
