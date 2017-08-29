import pytest

from indy.pool import refresh_pool_ledger


@pytest.mark.asyncio
async def test_refresh_pool_ledger_works(pool_handle):
    await refresh_pool_ledger(pool_handle)
