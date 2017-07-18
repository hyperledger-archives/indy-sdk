from indy import wallet, signus

from ..utils import storage
from ..utils.wallet import create_and_open_wallet

import asyncio
import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.yield_fixture()
def wallet_handle():
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    future = asyncio.Future()
    asyncio.ensure_future(create_and_open_wallet(future))
    loop.run_until_complete(future)
    yield future.result()
    loop.run_until_complete(wallet.close_wallet(future.result()))
    loop.close()


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

    result = json.loads((await signus.sign(wallet_handle, did.decode(), json.dumps(message))).decode())
    assert result['signature'] == expected_signature
