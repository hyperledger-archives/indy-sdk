import json

import base58
import pytest

from indy import error
from indy import did


@pytest.mark.asyncio
async def test_create_my_did_works_with_empty_json(wallet_handle):
    (_did, ver_key) = await did.create_and_store_my_did(wallet_handle, "{}")
    assert len(base58.b58decode(_did)) == 16
    assert len(base58.b58decode(ver_key)) == 32


@pytest.mark.asyncio
async def test_create_my_did_works_for_seed(wallet_handle, seed_my1, did_my1, verkey_my1):
    (_did, ver_key) = await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1}))
    assert did_my1 == _did
    assert verkey_my1 == ver_key


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_crypto_type(wallet_handle):
    with pytest.raises(error.UnknownCryptoTypeError):
        await did.create_and_store_my_did(wallet_handle, json.dumps({'crypto_type': 'crypto_type'}))


@pytest.mark.asyncio
async def test_create_my_did_works_for_duplicate(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, '{}')
    with pytest.raises(error.DidAlreadyExistsError):
        await did.create_and_store_my_did(wallet_handle, json.dumps({'did': _did}))
