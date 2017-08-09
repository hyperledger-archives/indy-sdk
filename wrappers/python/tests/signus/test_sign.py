from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import json
import pytest


@pytest.mark.asyncio
async def test_sign_works(wallet_handle, seed_trustee1):
    (did, _, _) = await signus.create_and_store_my_did(wallet_handle, json.dumps({"seed": seed_trustee1}))

    message = '{"reqId":1496822211362017764}'
    expected_signature = "R4Rj68n4HZosQqEc3oMUbQh7MtG8tH7WmXE2Mok8trHJ67CrzyqahZn5ziJy4nebRtq6Qi6fVH9JkvVCM85XjFa"

    signature = await signus.sign(wallet_handle, did, message)
    assert signature == expected_signature


@pytest.mark.asyncio
async def test_sign_works_for_unknown_did(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.sign(wallet_handle, '8wZcEriaNLNKtteJvx7f8i', json.dumps({"reqId": 1496822211362017764}))
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_sign_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{}')
        await signus.sign(wallet_handle + 1, did, json.dumps({"reqId": 1496822211362017764}))
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
