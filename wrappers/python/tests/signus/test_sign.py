from indy import IndyError
from indy import signus
from indy.error import ErrorCode

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_sign_works(wallet_handle):
    (did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"000000000000000000000000Trustee1"}')

    message = {
        "reqId": 1496822211362017764,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "VsKV7grR1BUE29mG2Fm2kX",
            "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
        }
    }

    expected_signature = "65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"

    result = json.loads(await signus.sign(wallet_handle, did, json.dumps(message)))
    assert result['signature'] == expected_signature


@pytest.mark.asyncio
async def test_sign_works_for_unknown_did(wallet_handle):
    with pytest.raises(IndyError) as e:
        message = {"reqId": 1496822211362017764}
        await signus.sign(wallet_handle, '8wZcEriaNLNKtteJvx7f8i', json.dumps(message))
    assert ErrorCode.WalletNotFoundError == e.value.error_code


@pytest.mark.asyncio
async def test_sign_works_for_invalid_message_format(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{}')
        message = '"reqId":1495034346617224651'
        await signus.sign(wallet_handle, did, message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_sign_works_for_invalid_handle(wallet_handle):
    with pytest.raises(IndyError) as e:
        (did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{}')
        message = {"reqId": 1496822211362017764}
        await signus.sign(wallet_handle + 1, did, json.dumps(message))
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
