import pytest

from indy import pool, error


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works(pool_ledger_config):
    pass


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_empty_name():
    with pytest.raises(error.CommonInvalidParam2):
        await pool.create_pool_ledger_config("", None)

