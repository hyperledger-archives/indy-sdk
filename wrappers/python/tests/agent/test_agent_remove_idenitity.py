import pytest

from indy import agent


@pytest.mark.asyncio
async def test_agent_remove_identity_works(listener_with_identity):
    listener_handle, wallet_handle, did = listener_with_identity
    await agent.agent_remove_identity(listener_handle, wallet_handle, did)
