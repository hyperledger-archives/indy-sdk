from indy.anoncreds import prover_get_claim_offers
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_empty_filter(init_common_wallet):
    claim_offers = json.loads(await prover_get_claim_offers(init_common_wallet[0], "{}"))
    assert len(claim_offers) == 3


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_issuer(init_common_wallet):
    claim_offers = json.loads(await prover_get_claim_offers(
        init_common_wallet[0], '{{"issuer_did":"{}"}}'.format(anoncreds.ISSUER_DID)))
    assert len(claim_offers) == 2
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 1} in claim_offers
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 2} in claim_offers


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_schema(init_common_wallet):
    claim_offers = json.loads(await prover_get_claim_offers(
        init_common_wallet[0], '{"schema_seq_no":2}'))
    assert len(claim_offers) == 2
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 2} in claim_offers
    assert {'issuer_did': 'CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW', 'schema_seq_no': 2} in claim_offers


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_issuer_and_schema(init_common_wallet):
    claim_offers = json.loads(await prover_get_claim_offers(
        init_common_wallet[0], json.dumps(anoncreds.get_claim_offer(anoncreds.ISSUER_DID, 1))))
    assert len(claim_offers) == 1
    assert {'issuer_did': anoncreds.ISSUER_DID, 'schema_seq_no': 1} in claim_offers


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_no_results(init_common_wallet):
    claim_offers = json.loads(await prover_get_claim_offers(
        init_common_wallet[0], '{"schema_seq_no":4}'))
    assert len(claim_offers) == 0


@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_invalid_wallet_handle(init_common_wallet):
    invalid_wallet_handle = init_common_wallet[0] + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claim_offers(invalid_wallet_handle, '{"schema_seq_no":1}')
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


