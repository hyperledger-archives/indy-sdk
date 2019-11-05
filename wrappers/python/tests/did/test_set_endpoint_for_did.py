import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_set_endpoint_for_did_works(wallet_handle, identity_trustee1, endpoint):
    (_did, verkey) = identity_trustee1
    await did.set_endpoint_for_did(wallet_handle, _did, endpoint, verkey)


@pytest.mark.asyncio
async def test_set_endpoint_for_did_works_for_invalid_did(wallet_handle, verkey_my1, endpoint):
    with pytest.raises(error.CommonInvalidStructure):
        await did.set_endpoint_for_did(wallet_handle, 'invalid_base58string', endpoint, verkey_my1)
