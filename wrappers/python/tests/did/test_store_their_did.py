import json

import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_store_their_did_works(wallet_handle, did_my):
    await did.store_their_did(wallet_handle, json.dumps({"did": did_my}))


@pytest.mark.asyncio
async def test_store_their_did_works_for_invalid_json(wallet_handle):
    with pytest.raises(error.CommonInvalidStructure):
        await did.store_their_did(wallet_handle, '{"field":"value"}')
