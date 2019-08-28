from indy import did

import pytest


@pytest.mark.asyncio
async def test_qualify_did(wallet_handle):
    prefix = 'did:peer'
    (did_, verkey_) = await did.create_and_store_my_did(wallet_handle, "{}")
    full_qualified_did = await did.qualify_did(wallet_handle, did_, prefix)
    expected_did = prefix + ':' + did_
    assert expected_did  == full_qualified_did