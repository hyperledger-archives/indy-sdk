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
async def test_create_wallet_works():
    pool_name = 'indy_create_wallet_works'
    wallet_name = 'indy_create_wallet_works'
    xtype = 'default'

    await wallet.create_wallet(pool_name, wallet_name, xtype, None, None)
