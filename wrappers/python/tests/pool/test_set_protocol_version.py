import pytest

from indy import pool
from indy.error import ErrorCode, IndyError


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_test_set_protocol_version_works(protocol_version):
    await pool.set_protocol_version(protocol_version)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_test_set_protocol_version_works_for_unsupported():
    with pytest.raises(IndyError) as e:
        await pool.set_protocol_version(0)

    assert ErrorCode.PoolIncompatibleProtocolVersion == e.value.error_code
