import json

from tests.utils import pool as pool_utils
from indy import pool
from indy.error import ErrorCode, IndyError

import pytest


@pytest.mark.asyncio
async def test_open_pool_ledger_works(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1"))
        }))

    pool_handle = await pool.open_pool_ledger("pool_1", None)
    assert pool_handle is not None

    await pool.close_pool_ledger(pool_handle)


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_config(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1"))
        }))

    pool_handle = await pool.open_pool_ledger("pool_1", '{"refreshOnOpen": true}')
    assert pool_handle is not None

    await pool.close_pool_ledger(pool_handle)


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_twice(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1"))
        }))

    pool_handle = await pool.open_pool_ledger("pool_1", None)
    assert pool_handle is not None

    with pytest.raises(IndyError) as e:
        await pool.open_pool_ledger("pool_1", "")

    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code
    await pool.close_pool_ledger(pool_handle)


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_two_nodes(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1", 2))
        }))

    pool_handle = await pool.open_pool_ledger("pool_1", None)
    assert pool_handle is not None

    await pool.close_pool_ledger(pool_handle)


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_three_nodes(cleanup_storage):
    await pool.create_pool_ledger_config(
        "pool_1",
        json.dumps({
            "genesis_txn": str(pool_utils.create_genesis_txn_file_for_test_pool("pool_1", 3))
        }))

    pool_handle = await pool.open_pool_ledger("pool_1", None)
    assert pool_handle is not None

    await pool.close_pool_ledger(pool_handle)
