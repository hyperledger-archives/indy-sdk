import json

from indysdk import IndyError
from indysdk import signus
from indysdk.error import ErrorCode

import base58
import pytest

seed = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
expected_verkey = 'CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW'
crypto_type = 'ed25519'
expected_did = 'NcYxiDXkpYi6ov5FcYDi1e'


@pytest.mark.asyncio
async def test_create_my_did_works_with_empty_json(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    assert len(base58.b58decode(did)) == 16
    assert len(base58.b58decode(ver_key)) == 32


@pytest.mark.asyncio
async def test_create_my_did_works_for_seed(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed}))
    assert expected_did == did
    assert expected_verkey == ver_key


@pytest.mark.asyncio
async def test_create_my_did_works_as_cid(wallet_handle):
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed, 'cid': True}))
    assert expected_verkey == did
    assert expected_verkey == ver_key


@pytest.mark.asyncio
async def test_create_my_did_works_for_passed_did(wallet_handle):
    (did, _, _) = await signus.create_and_store_my_did(wallet_handle, json.dumps({'did': expected_did}))
    assert expected_did == did


@pytest.mark.asyncio
async def test_create_my_did_works_for_correct_type(wallet_handle):
    (did, ver_key, _) = \
        await signus.create_and_store_my_did(wallet_handle, json.dumps({'seed': seed, 'crypto_type': crypto_type}))
    assert expected_did == did
    assert expected_verkey == ver_key


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_seed(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.create_and_store_my_did(wallet_handle, json.dumps({'seed': 'aaaaaaaaaaa'}))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_crypto_type(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.create_and_store_my_did(wallet_handle, json.dumps({'crypto_type': 'crypto_type'}))
    assert ErrorCode.SignusUnknownCryptoError == e.value.error_code


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.create_and_store_my_did(wallet_handle + 1, '{}')
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
