from indy import signus

import pytest


@pytest.mark.asyncio
async def test_replace_keys_works(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    (new_did, new_ver_key) = await signus.replace_keys(wallet_handle, did, "{}")
    assert (new_did != did) and (new_ver_key != ver_key)