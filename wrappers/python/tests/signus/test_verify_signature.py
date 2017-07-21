from indy import wallet, signus

from ..utils import storage
from ..utils.wallet import create_and_open_wallet

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.fixture
async def wallet_handle():
    handle = await create_and_open_wallet()
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_verify_signature_works(wallet_handle):
    pool_handle = 1
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"000000000000000000000000Trustee1"}')
    identity_json = {
        "did": did.decode(),
        "verkey": ver_key.decode()
    }

    await signus.store_their_did(wallet_handle, json.dumps(identity_json))

    message = {
        "reqId": 1496822211362017764,
        "identifier": "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation": {
            "type": "1",
            "dest": "VsKV7grR1BUE29mG2Fm2kX",
            "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
        },
        "signature": "65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW"
    }

    valid = await signus.verify_signature(wallet_handle, pool_handle, did.decode(), json.dumps(message))
    assert valid
