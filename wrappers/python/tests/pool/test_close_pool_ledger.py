import pytest

from indy import pool, error


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("pool_handle_cleanup", [False])
async def test_close_pool_ledger_works(pool_handle, pool_handle_cleanup):
    await pool.close_pool_ledger(pool_handle)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("pool_handle_cleanup", [False])
async def test_close_pool_ledger_works_for_twice(pool_handle, pool_handle_cleanup):
    await pool.close_pool_ledger(pool_handle)

    with pytest.raises(error.PoolLedgerInvalidPoolHandle):
        await pool.close_pool_ledger(pool_handle)



# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("pool_handle_cleanup", [False])
async def test_close_pool_ledger_works_for_reopen_after_close(pool_name, pool_config, pool_handle, pool_handle_cleanup):
    await pool.close_pool_ledger(pool_handle)

    pool_handle = await pool.open_pool_ledger(pool_name, pool_config)
    assert pool_handle is not None

    await pool.close_pool_ledger(pool_handle)
