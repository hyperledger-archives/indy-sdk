import pytest

from indy import pool, error


# noinspection PyUnusedLocal
@pytest.mark.asyncio
@pytest.mark.parametrize("pool_ledger_config_cleanup", [False])
async def test_delete_pool_ledger_config_works(pool_name, pool_ledger_config, pool_ledger_config_cleanup):
    await pool.delete_pool_ledger_config(pool_name)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works_for_opened(pool_name, pool_handle):
    with pytest.raises(error.CommonInvalidState):
        await pool.delete_pool_ledger_config(pool_name)
