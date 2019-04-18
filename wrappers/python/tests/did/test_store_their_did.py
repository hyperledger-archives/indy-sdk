import json

from indy import IndyError
from indy import did

import pytest

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_store_their_did_works(wallet_handle, did_my):
    await did.store_their_did(wallet_handle, json.dumps({"did": did_my}))


@pytest.mark.asyncio
async def test_store_their_did_works_for_invalid_json(wallet_handle):
    with pytest.raises(IndyError) as e:
        await did.store_their_did(wallet_handle, '{"field":"value"}')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_store_their_did_works_for_invalid_handle(wallet_handle, did_my):
    with pytest.raises(IndyError) as e:
        await did.store_their_did(wallet_handle + 1, json.dumps({"did": did_my}))
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_store_their_did_works_with_verkey(wallet_handle, did_my1, verkey_my1):
    await did.store_their_did(wallet_handle, json.dumps({"did": did_my1, "verkey": verkey_my1}))


@pytest.mark.asyncio
async def test_store_their_did_works_without_did(wallet_handle, verkey_my1):
    with pytest.raises(IndyError) as e:
        await did.store_their_did(wallet_handle, json.dumps({"verkey": verkey_my1}))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_store_their_did_works_for_invalid_did(wallet_handle):
    with pytest.raises(IndyError) as e:
        await did.store_their_did(wallet_handle, '{"did": "invalid_base58_string"}')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
