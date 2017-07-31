from tests.utils import pool
from indy.pool import delete_pool_ledger_config

import pytest


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works(cleanup_storage):
    name = "delete_pool_ledger_config_works"
    await pool.create_pool_ledger_config(name)
    await delete_pool_ledger_config(name)