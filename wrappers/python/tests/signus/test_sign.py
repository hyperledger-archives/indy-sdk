from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import json
import pytest


@pytest.mark.asyncio
async def test_sign_works(wallet_handle, seed_my1, message):
    (did, _) = await signus.create_and_store_my_did(wallet_handle, json.dumps({"seed": seed_my1}))

    expected_signature = bytes(
        [169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120,
         94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191,
         41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11])

    signature = await signus.sign(wallet_handle, did, message)
    assert signature == expected_signature


@pytest.mark.asyncio
async def test_sign_works_for_unknown_did(wallet_handle, message, did_my1):
    with pytest.raises(IndyError) as e:
        await signus.sign(wallet_handle, did_my1, message)
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_sign_works_for_invalid_handle(wallet_handle, message):
    with pytest.raises(IndyError) as e:
        (did, _) = await signus.create_and_store_my_did(wallet_handle, '{}')
        await signus.sign(wallet_handle + 1, did, message)
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
