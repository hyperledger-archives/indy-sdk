import pytest

from indy import wallet
from indy.error import IndyError, ErrorCode
from ..conftest import wallet_name


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize(
    "wallet_type, wallet_config",
    [('default', None),
     (None, None),
     ('default', '{"freshness_time":1000}')])
async def test_create_wallet_works(wallet_type, wallet_config, xwallet):
    pass


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize(
    "wallet_name, wallet_type, error_code",
    [(wallet_name(), "unknown_type", ErrorCode.WalletUnknownTypeError),
     ('', "default", ErrorCode.CommonInvalidParam3)])
async def test_create_wallet_works_for_exception(pool_name, wallet_name, wallet_type, error_code, path_home):
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet(pool_name, wallet_name, wallet_type, None, None)
    assert error_code == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_create_wallet_works_for_duplicate_name(pool_name, wallet_name, wallet_type, xwallet):
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet(pool_name, wallet_name, wallet_type, None, None)

    assert ErrorCode.WalletAlreadyExistsError == e.value.error_code


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize(
    "wallet_name, credentials",
    [(wallet_name(), '{"key":"testkey"}')])
async def test_create_wallet_works_for_credentials(xwallet, credentials):
    pass
