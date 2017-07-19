from indy import wallet, signus

from ..utils import storage
from ..utils.wallet import create_and_open_wallet

import base58
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
async def test_create_my_did_works_with_empty_json(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    assert len(base58.b58decode(did)) == 16
    assert len(base58.b58decode(ver_key)) == 32
