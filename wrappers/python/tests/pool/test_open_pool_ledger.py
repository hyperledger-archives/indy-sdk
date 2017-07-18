from tests.utils import pool, storage
from indy.pool import open_pool_ledger
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_open_pool_ledger_works():
    name = "pool_create"
    await pool.create_pool_ledger_config(name)
    pool_handle = await open_pool_ledger(name, "")
    assert pool_handle is not None
