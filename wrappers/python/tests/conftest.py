import logging

import pytest

from .utils import pool, storage, wallet, anoncreds

logging.basicConfig(level=logging.DEBUG)
WALLET = {
    "opened": False,
    "handle": None,
    "claim_def": None
}


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


# noinspection PyUnusedLocal
@pytest.fixture
async def init_common_wallet():
    global WALLET
    if WALLET["opened"]:
        yield (WALLET["handle"], WALLET["claim_def"])
        return

    storage.cleanup()
    wallet_handle = await wallet.create_and_open_wallet(pool_name="pool_2", wallet_name="wallet_2")
    assert type(wallet_handle) is int
    claim_def = await anoncreds.prepare_common_wallet(wallet_handle)

    WALLET = {
        "opened": True,
        "handle": wallet_handle,
        "claim_def": claim_def
    }
    yield (wallet_handle, claim_def)
