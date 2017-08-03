from tests.utils import pool, storage
from indy_sdk.error import ErrorCode, IndyError

import pytest


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works(cleanup_storage):
    await pool.create_pool_ledger_config("pool_1")


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_empty_name(cleanup_storage):
    with pytest.raises(IndyError) as e:
        await pool.create_pool_ledger_config("")
    assert ErrorCode.CommonInvalidParam2 == e.value.error_code


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_config_json(cleanup_storage):
    config = pool.create_default_pool_config("pool_1")
    await pool.create_pool_ledger_config("pool_1", None, config, None)


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_specific_config(cleanup_storage):
    gen_txn_file_name = "specific_filename.txn"
    config = {
        "genesis_txn": str(storage.indy_temp_path().joinpath(gen_txn_file_name))
    }
    await pool.create_pool_ledger_config("pool_1", None, config, gen_txn_file_name)
