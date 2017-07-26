from indy import wallet

from ..utils import storage

import pytest


@pytest.yield_fixture(autouse=True)
def cleanup_storage():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_create_wallet_works():
    await wallet.create_wallet('pool1', 'wallet1', 'default', None, None)
    assert True
