from indy import wallet
from indy.anoncreds import prover_store_claim_offer

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
async def test_prover_store_claim_offer_works(wallet_handle):
    claim_offer = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    await prover_store_claim_offer(wallet_handle, json.dumps(claim_offer))

