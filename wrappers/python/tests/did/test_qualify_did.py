from indy import did

import pytest


@pytest.mark.asyncio
async def test_qualify_did(wallet_handle):
    method = 'peer'
    (did_, verkey_) = await did.create_and_store_my_did(wallet_handle, "{}")
    full_qualified_did = await did.qualify_did(wallet_handle, did_, method)
    expected_did = 'did:' + method + ':' + did_
    assert expected_did  == full_qualified_did