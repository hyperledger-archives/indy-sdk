from tests.utils import pool, storage
from tests.utils.wallet import create_and_open_wallet
from indy import wallet, signus, ledger

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
    handle = await create_and_open_wallet(pool_name="pool_name")
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_sign_and_submit_request_works(wallet_handle):
    pool_handle = await pool.create_and_open_pool_ledger("pool_name")
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle, '{"seed":"00000000000000000000000000000My1"}')
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    nym_request = await ledger.build_nym_request(trustee_did.decode(), my_did.decode(), None, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did.decode(), nym_request.decode())
