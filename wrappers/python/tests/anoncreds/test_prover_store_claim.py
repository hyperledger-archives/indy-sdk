from indy import wallet
from indy.anoncreds import prover_create_and_store_claim_req, prover_store_claim,\
    prover_create_master_secret, issuer_create_claim

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
async def wallet_handle_and_claim_def():
    handle = await create_and_open_wallet()
    claim_def = await anoncreds.prepare_common_wallet(handle)
    yield (handle, claim_def)
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_prover_store_claim_works(wallet_handle_and_claim_def):
    prover_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    claim_offer_json = anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1)
    await prover_create_master_secret(wallet_handle_and_claim_def[0], anoncreds.COMMON_MASTER_SECRET_NAME_1)
    claim_req = await prover_create_and_store_claim_req(wallet_handle_and_claim_def[0], prover_did,
                                                        json.dumps(claim_offer_json), wallet_handle_and_claim_def[1],
                                                        anoncreds.COMMON_MASTER_SECRET_NAME_1)

    (_, claim_json) = await issuer_create_claim(wallet_handle_and_claim_def[0], claim_req,
                                                json.dumps(anoncreds.get_gvt_claim_json()), -1, -1)
    await prover_store_claim(wallet_handle_and_claim_def[0], claim_json)

