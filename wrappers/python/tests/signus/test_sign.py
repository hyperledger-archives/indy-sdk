from indy_sdk import IndyError
from indy_sdk import signus
from indy_sdk.error import ErrorCode

import json
import pytest


@pytest.mark.asyncio
async def test_sign_works(wallet_handle):
    (did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"000000000000000000000000Trustee1"}')

    message = json.dumps({
        "reqId": 1496822211362017764,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "VsKV7grR1BUE29mG2Fm2kX",
            "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
        }
    })

    expected_signature = "65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"

    signed_msg = json.loads(await signus.sign(wallet_handle, did, message))
    assert signed_msg['signature'] == expected_signature


@pytest.mark.asyncio
async def test_sign_works_for_unknown_did(wallet_handle):
    with pytest.raises(IndyError) as e:
        await signus.sign(wallet_handle, '8wZcEriaNLNKtteJvx7f8i', json.dumps({"reqId": 1496822211362017764}))
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_sign_works_for_invalid_message_format(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{}')
        await signus.sign(wallet_handle, did, '"reqId":1495034346617224651')
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_sign_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{}')
        await signus.sign(wallet_handle + 1, did, json.dumps({"reqId": 1496822211362017764}))
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
