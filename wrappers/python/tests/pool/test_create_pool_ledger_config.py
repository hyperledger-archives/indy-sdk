from tests.utils import pool

import pytest
import logging


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works(cleanup_storage):
    await pool.create_pool_ledger_config("create_pool_ledger_config_works")