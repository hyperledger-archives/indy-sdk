import pytest

from indy import IndyError
from indy import wallet
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_open_wallet_works(wallet_handle):
    pass


@pytest.mark.asyncio
async def test_open_wallet_works_for_not_created_wallet(wallet_config, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet(wallet_config, credentials)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_twice(wallet_handle, wallet_config, credentials):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet(wallet_config, credentials)

    assert ErrorCode.WalletAlreadyOpenedError == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_missed_key(xwallet, wallet_config):
    with pytest.raises(IndyError) as e:
        await wallet.open_wallet(wallet_config, "{}")
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_open_wallet_works_for_changing_credentials(wallet_config):
    await wallet.create_wallet(wallet_config, '{"key":"key"}')
    handle = await wallet.open_wallet(wallet_config, '{"key":"key", "rekey":"other_key"}')
    await wallet.close_wallet(handle)

    handle = await wallet.open_wallet(wallet_config, '{"key":"other_key"}')
    await wallet.close_wallet(handle)
