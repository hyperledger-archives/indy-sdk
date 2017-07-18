from indy import wallet, signus

from ..utils import storage
from ..utils.wallet import create_and_open_wallet

import asyncio
import base58
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
async def test_create_my_did_works_with_empty_json(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    assert len(base58.b58decode(did)) == 16
    assert len(base58.b58decode(ver_key)) == 32
