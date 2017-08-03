import json

from tests.utils import pool as pool_utils
from indy import pool
from indy.error import ErrorCode, IndyError

import pytest


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1"))
        }))

    await pool.delete_pool_ledger_config("pool_1")


@pytest.mark.asyncio
async def test_delete_pool_ledger_config_works_for_opened(cleanup_storage):
    pool_handle = await pool_utils.create_and_open_pool_ledger("pool_1")

    with pytest.raises(IndyError) as e:
        await pool.delete_pool_ledger_config("pool_1")

    assert ErrorCode.CommonInvalidState == e.value.error_code
    await pool.close_pool_ledger(pool_handle)
