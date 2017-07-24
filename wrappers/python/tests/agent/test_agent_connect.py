import pytest


@pytest.mark.asyncio
async def test_agent_connect_works(connection):
    assert connection is not None
