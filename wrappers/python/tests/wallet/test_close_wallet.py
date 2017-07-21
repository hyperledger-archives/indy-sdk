from indy import wallet

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
    await wallet.create_wallet('pool1', 'wallet1', None, None, None)

    wallet_handle = await wallet.open_wallet('wallet1', None, None)
    await wallet.close_wallet(wallet_handle)

    wallet_handle = await wallet.open_wallet('wallet1', None, None)
    await wallet.close_wallet(wallet_handle)

    assert True
