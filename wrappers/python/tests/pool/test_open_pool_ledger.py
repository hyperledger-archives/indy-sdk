from tests.utils import pool
from indy.pool import open_pool_ledger

import pytest


@pytest.mark.asyncio
async def test_open_pool_ledger_works(cleanup_storage):
    name = "open_pool_ledger_works"
    await pool.create_pool_ledger_config(name)
    pool_handle = await open_pool_ledger(name, "")
    assert pool_handle is not None