from indy import wallet, signus

from ..utils import storage
from ..utils.wallet import create_and_open_wallet

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.fixture
async def wallet_handle():
    handle = await create_and_open_wallet()
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_store_their_did_works(wallet_handle):
    await signus.store_their_did(wallet_handle, '{"did":"8wZcEriaNLNKtteJvx7f8i"}')
