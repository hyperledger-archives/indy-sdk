import logging

import pytest

from .utils import storage, wallet

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture
def cleanup_storage():
    storage.cleanup()
    yield
    storage.cleanup()


# noinspection PyUnusedLocal
@pytest.fixture
async def wallet_handle(cleanup_storage):
    wallet_handle = await wallet.create_and_open_wallet()
    assert type(wallet_handle) is int
    yield wallet_handle
    await wallet.close_wallet(wallet_handle)
