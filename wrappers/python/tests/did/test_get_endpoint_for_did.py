import json

import time

from indy import IndyError
from indy import did, ledger
from indy.error import ErrorCode

import pytest


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works(wallet_handle, identity_trustee1, endpoint):
    (_did, key) = identity_trustee1
    await did.set_endpoint_for_did(wallet_handle, _did, endpoint, key)
    received_endpoint, received_key = await did.get_endpoint_for_did(wallet_handle, -1, _did)
    assert endpoint == received_endpoint
    assert key == received_key


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works_from_ledger(pool_handle, wallet_handle, identity_trustee1, endpoint):
    (_did, key) = identity_trustee1

    endpoint_json = json.dumps({"endpoint": {"ha": endpoint, "verkey": key}})
    attrib_request = await ledger.build_attrib_request(_did, _did, None, endpoint_json, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, _did, attrib_request)

    time.sleep(1)

    received_endpoint, received_key = await did.get_endpoint_for_did(wallet_handle, pool_handle, _did)
    assert endpoint == received_endpoint
    assert key == received_key


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works_for_unknown_did(pool_handle, wallet_handle, did_my1):
    with pytest.raises(IndyError) as e:
        await did.get_endpoint_for_did(wallet_handle, pool_handle, did_my1)
    assert ErrorCode.CommonInvalidState == e.value.error_code


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works_invalid_wallet_handle(pool_handle, wallet_handle,
                                                                identity_trustee1, endpoint):
    (_did, key) = identity_trustee1
    await did.set_endpoint_for_did(wallet_handle, _did, endpoint, key)
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await did.get_endpoint_for_did(invalid_wallet_handle, pool_handle, _did)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_get_endpoint_for_did_works_invalid_pool_handle(pool_handle, wallet_handle, identity_trustee1, endpoint):
    (_did, key) = identity_trustee1
    with pytest.raises(IndyError) as e:
        invalid_pool_handle = pool_handle + 1
        await did.get_endpoint_for_did(wallet_handle, invalid_pool_handle, _did)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code
