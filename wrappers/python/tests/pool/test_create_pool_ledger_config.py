from tests.utils import pool, storage

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works():
    await pool.create_pool_ledger_config("pool_create")
