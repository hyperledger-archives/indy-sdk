import json

from indy import IndyError
from indy import did
from indy.error import ErrorCode

import base58
import pytest


@pytest.mark.asyncio
async def test_list_my_dids_works(wallet_handle, seed_my1, did_my1, verkey_my1, metadata):
    await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1}))
    await did.set_did_metadata(wallet_handle, did_my1, metadata)

    res_json = await did.list_my_dids_with_meta(wallet_handle)
    res = json.loads(res_json)

    assert len(res) == 1
    assert res[0]["did"] == did_my1
    assert res[0]["metadata"] == metadata
    assert res[0]["verkey"] == verkey_my1

@pytest.mark.asyncio
async def test_list_my_dids_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        await did.list_my_dids_with_meta(wallet_handle + 1)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
