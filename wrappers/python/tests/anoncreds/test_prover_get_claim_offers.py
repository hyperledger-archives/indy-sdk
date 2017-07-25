from indy import wallet
from indy.anoncreds import prover_get_claim_offers
from indy.error import ErrorCode, IndyError

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


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_issuer(wallet_handle):
    claim_offers = json.loads(await prover_get_claim_offers(
        wallet_handle, '{{"issuer_did":"{}"}}'.format(anoncreds.ISSUER_DID)))
    assert len(claim_offers) == 2
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 1} in claim_offers
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 2} in claim_offers


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_schema(wallet_handle):
    claim_offers = json.loads(await prover_get_claim_offers(
        wallet_handle, '{"schema_seq_no":2}'))
    assert len(claim_offers) == 2
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 2} in claim_offers
    assert {'issuer_did': 'CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW', 'schema_seq_no': 2} in claim_offers


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_issuer_and_schema(wallet_handle):
    claim_offers = json.loads(await prover_get_claim_offers(
        wallet_handle, json.dumps(anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1))))
    assert len(claim_offers) == 1
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 1} in claim_offers


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_no_results(wallet_handle):
    claim_offers = json.loads(await prover_get_claim_offers(
        wallet_handle, '{"schema_seq_no":4}'))
    assert len(claim_offers) == 0


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_invalid_wallet_handle(wallet_handle):
    invalid_wallet_handle = wallet_handle + 100

    try:
        await prover_get_claim_offers(invalid_wallet_handle, '{"schema_seq_no":1}')
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.WalletInvalidHandle)) == type(e) and \
               IndyError(ErrorCode.WalletInvalidHandle).args == e.args

