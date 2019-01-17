import json

from indy import IndyError
from indy import did
from indy.error import ErrorCode

import base58
import pytest


@pytest.mark.asyncio
async def test_get_my_did_works(wallet_handle, seed_my1, did_my1, verkey_my1, metadata):
    await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1}))
    await did.set_did_metadata(wallet_handle, did_my1, metadata)

    res_json = await did.get_my_did_with_meta(wallet_handle, did_my1)
    res = json.loads(res_json)

    assert res["did"] == did_my1
    assert res["metadata"] == metadata
    assert res["verkey"] == verkey_my1

@pytest.mark.asyncio
async def test_get_my_dids_works_for_invalid_handle(wallet_handle, did_my1):
    with pytest.raises(IndyError) as e:
        await did.get_my_did_with_meta(wallet_handle + 1, did_my1)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code

@pytest.mark.asyncio
async def test_get_my_did_with_metadata_works_for_no_metadata(wallet_handle, did_my1):
    with pytest.raises(IndyError) as e:
        await did.get_my_did_with_meta(wallet_handle, did_my1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code
