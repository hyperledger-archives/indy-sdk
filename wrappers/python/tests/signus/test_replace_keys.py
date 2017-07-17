from indy import wallet, signus

from ..utils import storage

import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.yield_fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_replace_keys_works():
    pool_name = "indy_open_wallet_works"
    wallet_name = "indy_open_wallet_works"

    await wallet.create_wallet(pool_name, wallet_name, None, None, None)
    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    assert wallet_handle is not None
    (did, ver_key, _) = await signus.create_and_store_my_did(wallet_handle, "{}")
    (new_did, new_ver_key) = await signus.replace_keys(wallet_handle, did.decode(), "{}")
    assert (new_did != did) and (new_ver_key != ver_key)