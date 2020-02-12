import pytest
from indy.anoncreds import to_unqualified


@pytest.mark.asyncio
async def test_to_unqualified_works():
    qualified = "did:sov:NcYxiDXkpYi6ov5FcYDi1e"
    unqualified = "NcYxiDXkpYi6ov5FcYDi1e"
    assert unqualified == await to_unqualified(qualified)
    assert unqualified == await to_unqualified(unqualified)
