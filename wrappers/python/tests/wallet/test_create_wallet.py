from indy import IndyError
from indy import wallet
from indy.error import ErrorCode

from ..utils import storage

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def cleanup_storage():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_create_wallet_works():
    await wallet.create_wallet('pool1', 'wallet1', 'default', None, None)


@pytest.mark.asyncio
async def test_create_wallet_works_for_empty_type():
    await wallet.create_wallet('pool1', 'wallet1', None, None, None)


@pytest.mark.asyncio
async def test_create_wallet_works_for_config_json():
    await wallet.create_wallet('pool1', 'wallet3', 'default', '{"freshness_time":1000}', None)


@pytest.mark.asyncio
async def test_create_wallet_works_for_unknown_type():
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet('pool1', 'wallet3', 'unknown_type', None, None)
    assert ErrorCode.WalletUnknownTypeError == e.value.error_code


@pytest.mark.asyncio
async def test_create_wallet_works_for_empty_name():
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet('pool1', '', 'default', None, None)
    assert ErrorCode.CommonInvalidParam3 == e.value.error_code


@pytest.mark.asyncio
async def test_create_wallet_works_for_duplicate_name():
    with pytest.raises(IndyError) as e:
        await wallet.create_wallet('pool1', 'wallet4', 'default', None, None)
        await wallet.create_wallet('pool1', 'wallet4', 'default', None, None)
    assert ErrorCode.WalletAlreadyExistsError == e.value.error_code
