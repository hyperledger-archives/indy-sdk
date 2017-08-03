import json

from indy import pool
from tests.utils import pool as pool_utils, storage as storage_utils
from indy.error import ErrorCode, IndyError

import pytest


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1"))
        }))


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_empty_name(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await pool.create_pool_ledger_config("", None)
    assert ErrorCode.CommonInvalidParam2 == e.value.error_code


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_config_json(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1"))
        }))


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_specific_config(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(
                pool_utils.create_genesis_txn_file_for_test_pool(
                    "pool_1",
                    4,
                    storage_utils.indy_temp_path().joinpath("specific_filename.txn")))
        }))
