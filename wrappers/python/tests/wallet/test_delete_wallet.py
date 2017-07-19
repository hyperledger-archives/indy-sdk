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
async def test_delete_wallet_works():
    await wallet.create_wallet('pool1', 'wallet1', None, None, None)
    await wallet.delete_wallet('wallet1', None)
    await wallet.create_wallet('pool1', 'wallet1', None, None, None)

    assert True
