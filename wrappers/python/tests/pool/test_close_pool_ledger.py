from tests.utils import pool, storage
from indy.pool import close_pool_ledger
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_close_pool_ledger_works():
    handle = await pool.create_and_open_pool_ledger("pool_1")
    await close_pool_ledger(handle)
