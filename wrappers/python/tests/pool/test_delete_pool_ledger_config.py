from tests.utils import pool
from indysdk.pool import delete_pool_ledger_config
from indysdk.error import ErrorCode, IndyError

import pytest


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works(cleanup_storage):
    await pool.create_pool_ledger_config("pool_1")
    await delete_pool_ledger_config("pool_1")


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works_for_opened(cleanup_storage):
    pool_handle = await pool.create_and_open_pool_ledger("pool_1")
    with pytest.raises(IndyError) as e:
        await delete_pool_ledger_config("pool_1")
    assert ErrorCode.CommonInvalidState == e.value.error_code
    await pool.close_pool_ledger(pool_handle)

