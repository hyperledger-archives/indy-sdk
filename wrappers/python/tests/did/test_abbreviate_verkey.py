import json

from indy import did

import pytest


@pytest.mark.asyncio
async def test_abbreviate_verkey_for_abbbr_key(wallet_handle):
    (_did, full_verkey) = await did.create_and_store_my_did(wallet_handle, "{}")
    verkey = await did.abbreviate_verkey(_did, full_verkey)
    assert not full_verkey == verkey


@pytest.mark.asyncio
async def test_abbreviate_verkey_for_abbbr_key(wallet_handle, did_my1):
    (_did, full_verkey) = await did.create_and_store_my_did(wallet_handle, json.dumps({'did': did_my1}))
    verkey = await did.abbreviate_verkey(_did, full_verkey)
    assert full_verkey == verkey
