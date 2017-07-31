from tests.utils import pool
from indy.pool import close_pool_ledger

import pytest


@pytest.mark.asyncio
async def test_close_pool_ledger_works(cleanup_storage):
    handle = await pool.create_and_open_pool_ledger("close_pool_ledger_works")
    await close_pool_ledger(handle)