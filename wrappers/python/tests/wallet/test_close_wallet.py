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
async def test_close_wallet_works():
    wallet_name = 'wallet1'
    await wallet.create_wallet('pool1', wallet_name, None, None, None)

    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    await wallet.close_wallet(wallet_handle)

    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
async def test_close_wallet_works_for_twice():
    with pytest.raises(IndyError) as e:
        wallet_name = 'wallet_for_twice'
        await wallet.create_wallet('pool1', wallet_name, None, None, None)

        wallet_handle = await wallet.open_wallet(wallet_name, None, None)
        await wallet.close_wallet(wallet_handle)
        await wallet.close_wallet(wallet_handle)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
