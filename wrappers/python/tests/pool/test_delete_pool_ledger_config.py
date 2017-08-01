from tests.utils import pool
from indy.pool import delete_pool_ledger_config
from indy.error import ErrorCode, IndyError

import pytest


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works(cleanup_storage):
    name = "delete_pool_ledger_config_works"
    await pool.create_pool_ledger_config(name)
    await delete_pool_ledger_config(name)


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works_for_opened(cleanup_storage):
    name = "delete_pool_ledger_config_works_for_opened"
    await pool.create_and_open_pool_ledger(name)
    with pytest.raises(IndyError) as e:
        await delete_pool_ledger_config(name)
    assert ErrorCode.CommonInvalidState == e.value.error_code

