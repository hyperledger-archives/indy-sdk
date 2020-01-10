import pytest

from indy import pool, error


@pytest.mark.parametrize(
    "pool_genesis_txn_count, pool_config",
    [(2, None), (3, None), (4, None), (4, '{"timeout": 20}')])
@pytest.mark.asyncio
async def test_open_pool_ledger_works(pool_handle):
    pass


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_twice(pool_name, pool_config, pool_handle):
    with pytest.raises(error.PoolLedgerInvalidPoolHandle):
        await pool.open_pool_ledger(pool_name, pool_config)


@pytest.mark.asyncio
async def test_open_pool_ledger_works_for_incompatible_protocol_version(pool_ledger_config, pool_name,
                                                                        protocol_version):
    await pool.set_protocol_version(1)

    with pytest.raises(error.PoolIncompatibleProtocolVersion):
        await pool.open_pool_ledger(pool_name, None)

    await pool.set_protocol_version(protocol_version)
