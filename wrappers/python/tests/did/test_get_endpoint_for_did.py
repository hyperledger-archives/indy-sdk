import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works(wallet_handle, identity_trustee1, endpoint):
    (_did, key) = identity_trustee1
    await did.set_endpoint_for_did(wallet_handle, _did, endpoint, key)
    received_endpoint, received_key = await did.get_endpoint_for_did(wallet_handle, -1, _did)
    assert endpoint == received_endpoint
    assert key == received_key


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works_for_unknown_did(pool_handle, wallet_handle, did_my1):
    with pytest.raises(error.CommonInvalidState):
        await did.get_endpoint_for_did(wallet_handle, pool_handle, did_my1)
