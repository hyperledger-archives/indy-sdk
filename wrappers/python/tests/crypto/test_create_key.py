import json

import base58
import pytest

from indy import did, error


@pytest.mark.asyncio
async def test_create_key_works_for_seed(wallet_handle, seed_my1):
    ver_key = await did.create_key(wallet_handle, json.dumps({'seed': seed_my1}))
    assert len(base58.b58decode(ver_key)) == 32


@pytest.mark.asyncio
async def test_create_key_works_without_seed(wallet_handle):
    ver_key = await did.create_key(wallet_handle, "{}")
    assert len(base58.b58decode(ver_key)) == 32


@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_seed(wallet_handle):
    with pytest.raises(error.CommonInvalidStructure):
        await did.create_key(wallet_handle, json.dumps({'seed': 'invalidSeedLength'}))
