import pytest


@pytest.mark.asyncio
async def test_agent_listen_works(listener_handle):
    assert listener_handle is not None
