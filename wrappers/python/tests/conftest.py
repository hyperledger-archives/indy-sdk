import logging

import pytest

from .utils import pool, storage, wallet

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture
def trustee1_seed():
    return "000000000000000000000000Trustee1"


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


# noinspection PyUnusedLocal
@pytest.fixture
async def pool_handle(cleanup_storage):
    pool_handle = await pool.create_and_open_pool_ledger()
    assert type(pool_handle) is int
    yield pool_handle
    await pool.close_pool_ledger(pool_handle)
