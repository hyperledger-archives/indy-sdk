import pytest

from indy import pool
from indy.error import ErrorCode, IndyError


@pytest.mark.parametrize(
    "pool_genesis_txn_count, pool_config",
    [(2, None), (3, None), (4, None), (4, '{"timeout": 20}')])
@pytest.mark.asyncio
async def test_open_pool_ledger_works(pool_handle):
    pass


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_twice(pool_name, pool_config, pool_handle):
    with pytest.raises(IndyError) as e:
        await pool.open_pool_ledger(pool_name, pool_config)

    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_incompatible_protocol_version(pool_ledger_config, pool_name,
                                                                        protocol_version):
    await pool.set_protocol_version(1)

    with pytest.raises(IndyError) as e:
        await pool.open_pool_ledger(pool_name, None)

    assert ErrorCode.PoolIncompatibleProtocolVersion == e.value.error_code

    await pool.set_protocol_version(protocol_version)
