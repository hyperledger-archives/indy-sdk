from tests.utils import pool, storage
from indy.pool import delete_pool_ledger_config
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works():
    name = "pool_name"
    await pool.create_pool_ledger_config(name)
    await delete_pool_ledger_config(name)
