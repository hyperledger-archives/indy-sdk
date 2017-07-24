from indy import wallet
from indy.anoncreds import prover_get_claim_offers

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
async def test_prover_get_claim_offers_works_for_empty_filter(wallet_handle):
    claim_offers = json.loads(await prover_get_claim_offers(wallet_handle, "{}"))
    assert len(claim_offers) == 3

