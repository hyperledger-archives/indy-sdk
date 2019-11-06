import pytest

from indy import pool, error


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_test_set_protocol_version_works(protocol_version):
    await pool.set_protocol_version(protocol_version)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_test_set_protocol_version_works_for_unsupported():
    with pytest.raises(error.PoolIncompatibleProtocolVersion):
        await pool.set_protocol_version(0)
