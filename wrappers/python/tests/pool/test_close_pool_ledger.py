from tests.utils import pool
from indy.pool import close_pool_ledger, open_pool_ledger
from indy.error import ErrorCode, IndyError

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_close_pool_ledger_works(cleanup_storage):
    handle = await pool.create_and_open_pool_ledger("pool_1")
    await close_pool_ledger(handle)


@pytest.mark.asyncio
async def test_close_pool_ledger_works_for_twice(cleanup_storage):
    handle = await pool.create_and_open_pool_ledger("pool_1")
    await close_pool_ledger(handle)

    with pytest.raises(IndyError) as e:
        await close_pool_ledger(handle)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_close_pool_ledger_works_for_reopen_after_close(cleanup_storage):
    pool_name = "close_pool_ledger_works_for_reopen_after_close"
    handle = await pool.create_and_open_pool_ledger(pool_name)
    await close_pool_ledger(handle)
    handle = await open_pool_ledger(pool_name, None)
    assert handle is not None


