import pytest


@pytest.mark.asyncio
async def test_agent_add_identity_works(listener_with_identity):
    assert listener_with_identity is not None


@pytest.mark.asyncio
async def test_agent_add_identity_works_for_multiply_keys(listener_with_identities):
    assert listener_with_identities is not None
