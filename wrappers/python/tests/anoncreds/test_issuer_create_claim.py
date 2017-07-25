from indy import wallet
from indy.anoncreds import issuer_create_claim

from tests.utils import storage, anoncreds
from tests.utils.wallet import create_and_open_wallet

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
    await anoncreds.prepare_common_wallet(handle)
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_issuer_create_claim_works(wallet_handle):
    claim_req = anoncreds.get_claim_req()
    claim_json = anoncreds.get_gvt_claim_json()
    (_, claim_json) = await issuer_create_claim(wallet_handle, json.dumps(claim_req), json.dumps(claim_json), -1, -1)

