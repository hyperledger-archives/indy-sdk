import json

from indy.error import ErrorCode, IndyError
from tests.utils import pool as pool_utils
from indy import pool

import pytest

from tests.utils.pool import create_genesis_txn_file


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works(cleanup_storage):
    await pool_utils.create_pool_ledger_config("pool_create")\


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_twice(cleanup_storage):
    with pytest.raises(IndyError) as e:
        path = create_genesis_txn_file('pool_1.txn', None)
        pool_config = json.dumps({"genesis_txn": str(path)})

        await pool.create_pool_ledger_config('pool_1', pool_config)
        await pool.create_pool_ledger_config('pool_1', pool_config)
    assert ErrorCode.PoolLedgerAlreadyExistsError == e.value.error_code


