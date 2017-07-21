from indy import wallet

from ..utils import storage

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_open_wallet_works():
    await wallet.create_wallet('pool1', 'wallet1', None, None, None)
    wallet_handle = await wallet.open_wallet('wallet1', None, None)
    assert wallet_handle is not None

    await wallet.close_wallet(wallet_handle)
    assert True
