import pytest

from indy import pool
from indy.error import ErrorCode, IndyError


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works(pool_ledger_config):
    pass


@pytest.mark.asyncio
async def test_create_pool_ledger_config_works_for_empty_name():
    with pytest.raises(IndyError) as e:
        await pool.create_pool_ledger_config("", None)

    assert ErrorCode.CommonInvalidParam2 == e.value.error_code
