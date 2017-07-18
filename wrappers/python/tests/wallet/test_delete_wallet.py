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
async def test_delete_wallet_works():
    pool_name = "indy_delete_wallet_works"
    wallet_name = "indy_delete_wallet_works"

    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    await wallet.delete_wallet(wallet_name, None)
    await wallet.create_wallet(pool_name, wallet_name, None, None, None)

    assert True
