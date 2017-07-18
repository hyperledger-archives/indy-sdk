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
async def test_replace_keys_works(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    (new_did, new_ver_key) = await signus.replace_keys(wallet_handle, did.decode(), "{}")
    assert (new_did != did) and (new_ver_key != ver_key)