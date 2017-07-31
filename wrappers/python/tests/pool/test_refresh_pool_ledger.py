from tests.utils import pool
from indy.pool import refresh_pool_ledger
import pytest


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_refresh_pool_ledger_works():
    handle = await pool.create_and_open_pool_ledger("refresh_pool_ledger_works")
    await refresh_pool_ledger(handle)