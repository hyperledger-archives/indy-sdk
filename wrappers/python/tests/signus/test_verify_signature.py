from indy import wallet, signus

from ..utils import storage

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_verify_signature_works():
    pool_name = "indy_open_wallet_works"
    pool_handle = 1
    wallet_name = "indy_open_wallet_works"

    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    assert wallet_handle is not None
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
