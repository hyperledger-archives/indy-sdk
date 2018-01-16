import json

from indy import IndyError
from indy import did
from indy.error import ErrorCode

import base58
import pytest


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
async def test_create_my_did_works_as_cid(wallet_handle, seed_my1, verkey_my1):
    (_did, ver_key) = await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1, 'cid': True}))
    assert verkey_my1 == _did
    assert verkey_my1 == ver_key


@pytest.mark.asyncio
async def test_create_my_did_works_for_passed_did(wallet_handle, did_my1):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, json.dumps({'did': did_my1}))
    assert did_my1 == _did


@pytest.mark.asyncio
async def test_create_my_did_works_for_correct_type(wallet_handle, seed_my1, did_my1, verkey_my1, crypto_type):
    (_did, ver_key) = \
        await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed_my1, 'crypto_type': crypto_type}))
    assert did_my1 == _did
    assert verkey_my1 == ver_key


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_seed(wallet_handle):
    with pytest.raises(IndyError) as e:
        await did.create_and_store_my_did(wallet_handle, json.dumps({'seed': 'aaaaaaaaaaa'}))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_crypto_type(wallet_handle):
    with pytest.raises(IndyError) as e:
        await did.create_and_store_my_did(wallet_handle, json.dumps({'crypto_type': 'crypto_type'}))
    assert ErrorCode.UnknownCryptoTypeError == e.value.error_code


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        await did.create_and_store_my_did(wallet_handle + 1, '{}')
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


@pytest.mark.asyncio
async def test_create_my_did_works_for_duplicate(wallet_handle):
    (_did, _) = await did.create_and_store_my_did(wallet_handle, '{}')
    with pytest.raises(IndyError) as e:
        await did.create_and_store_my_did(wallet_handle, json.dumps({'did': _did}))
    assert ErrorCode.DidAlreadyExistsError == e.value.error_code
