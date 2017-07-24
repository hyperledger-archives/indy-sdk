from indy import wallet
from indy.anoncreds import prover_create_master_secret

from tests.utils import storage, anoncreds
from tests.utils.wallet import create_and_open_wallet

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
    await anoncreds.prepare_common_wallet(handle)
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_prover_create_master_secret_works(wallet_handle):
    await prover_create_master_secret(wallet_handle, "master_secret_name")

