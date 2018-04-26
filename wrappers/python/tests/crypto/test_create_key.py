import json

from indy import IndyError
from indy import did
from indy.error import ErrorCode

import base58
import pytest


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
    with pytest.raises(IndyError) as e:
        await did.create_key(wallet_handle, json.dumps({'seed': 'invalidSeedLength'}))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code



@pytest.mark.asyncio
async def test_create_my_did_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        invalid_wallet_handle = wallet_handle + 1
        await did.create_key(invalid_wallet_handle, '{}')
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
