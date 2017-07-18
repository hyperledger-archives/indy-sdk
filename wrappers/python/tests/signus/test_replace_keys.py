from indy import wallet, signus

from ..utils import storage
from ..utils.wallet import create_and_open_wallet

import asyncio
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.yield_fixture()
def wallet_handle():
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    future = asyncio.Future()
    asyncio.ensure_future(create_and_open_wallet(future))
    loop.run_until_complete(future)
    yield future.result()
    loop.run_until_complete(wallet.close_wallet(future.result()))
    loop.close()


@pytest.mark.asyncio
async def test_replace_keys_works(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    (new_did, new_ver_key) = await signus.replace_keys(wallet_handle, did.decode(), "{}")
    assert (new_did != did) and (new_ver_key != ver_key)