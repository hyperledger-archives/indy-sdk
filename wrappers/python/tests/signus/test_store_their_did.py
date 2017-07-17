from indy import wallet, signus

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
async def test_store_their_did_works():
    pool_name = "indy_open_wallet_works"
    wallet_name = "indy_open_wallet_works"

    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    assert wallet_handle is not None
    await signus.store_their_did(wallet_handle, '{"did":"8wZcEriaNLNKtteJvx7f8i"}')
