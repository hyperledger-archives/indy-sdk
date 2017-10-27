from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_get_endpoint_works(wallet_handle, identity_trustee1, endpoint):
    (did, key) = identity_trustee1
    await signus.set_endpoint_for_did(wallet_handle, did, endpoint, key)
    received_endpoint, received_key = await signus.get_endpoint_for_did(wallet_handle, did)
    assert endpoint == received_endpoint
    assert key == received_key


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works_for_unknown_did(wallet_handle, identity_trustee1):
    (did, _) = identity_trustee1
    with pytest.raises(IndyError) as e:
        await signus.get_endpoint_for_did(wallet_handle, did)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works_invalid_handle(wallet_handle, identity_trustee1, endpoint):
    (did, key) = identity_trustee1
    await signus.set_endpoint_for_did(wallet_handle, did, endpoint, key)
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await signus.get_endpoint_for_did(invalid_wallet_handle, did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
