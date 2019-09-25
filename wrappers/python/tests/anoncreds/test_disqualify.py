import pytest
from indy.anoncreds import disqualify


@pytest.mark.asyncio
async def test_disqualify_works():
    qualified = "did:sov:NcYxiDXkpYi6ov5FcYDi1e"
    unqualified = "NcYxiDXkpYi6ov5FcYDi1e"
    assert unqualified == await disqualify(qualified)
    assert unqualified == await disqualify(unqualified)
