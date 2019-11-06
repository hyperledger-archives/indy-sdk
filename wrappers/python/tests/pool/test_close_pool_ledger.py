import pytest

from indy import pool


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("pool_handle_cleanup", [False])
async def test_close_pool_ledger_works(pool_handle, pool_handle_cleanup):
    await pool.close_pool_ledger(pool_handle)