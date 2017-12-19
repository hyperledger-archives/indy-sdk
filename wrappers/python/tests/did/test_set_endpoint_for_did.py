from indy import IndyError
from indy import did
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_set_endpoint_for_did_works(wallet_handle, identity_trustee1, endpoint):
    (_did, verkey) = identity_trustee1
    await did.set_endpoint_for_did(wallet_handle, _did, endpoint, verkey)


@pytest.mark.asyncio
async def test_set_endpoint_for_did_works_for_replace(pool_handle,wallet_handle, identity_trustee1, endpoint):
    (_did, key) = identity_trustee1
    await did.set_endpoint_for_did(wallet_handle, _did, endpoint, key)
    received_endpoint, received_key = await did.get_endpoint_for_did(wallet_handle, pool_handle, _did)
    assert endpoint == received_endpoint
    assert key == received_key

    new_endpoint = '100.0.0.1:9710'
    new_key = 'CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW'
    await did.set_endpoint_for_did(wallet_handle, _did, new_endpoint, new_key)
    updated_endpoint, updated_key = await did.get_endpoint_for_did(wallet_handle, pool_handle, _did)
    assert new_endpoint == updated_endpoint
    assert new_key == updated_key


@pytest.mark.asyncio
async def test_set_endpoint_for_did_works_for_invalid_did(wallet_handle, verkey_my1, endpoint):
    with pytest.raises(IndyError) as e:
        await did.set_endpoint_for_did(wallet_handle, 'invalid_base58string', endpoint, verkey_my1)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_set_endpoint_for_did_works_for_invalid_transport_key(wallet_handle, identity_trustee1, endpoint):
    (_did, _) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await did.set_endpoint_for_did(wallet_handle, _did, endpoint, 'CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_set_endpoint_for_did_works_for_invalid_handle(wallet_handle, identity_trustee1, endpoint):
    (_did, key) = identity_trustee1
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await did.set_endpoint_for_did(invalid_wallet_handle, _did, endpoint, key)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
