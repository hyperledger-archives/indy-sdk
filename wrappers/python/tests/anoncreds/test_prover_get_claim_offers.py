from indy.anoncreds import prover_get_claim_offers
from indy.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_empty_filter(wallet_handle, prepopulated_wallet):
    claim_offers = json.loads(
        await prover_get_claim_offers(wallet_handle, "{}"))

    assert len(claim_offers) == 3


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_issuer(wallet_handle, prepopulated_wallet, issuer_did):
    claim_offers = json.loads(
        await prover_get_claim_offers(wallet_handle, json.dumps({"issuer_did": issuer_did})))

    assert len(claim_offers) == 2
    assert {"issuer_did": issuer_did, "schema_seq_no": 1} in claim_offers
    assert {"issuer_did": issuer_did, "schema_seq_no": 2} in claim_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_schema(wallet_handle, prepopulated_wallet, issuer_did,
                                                                  prover_did,
                                                                  schema_seq_no_2):
    claim_offers = json.loads(
        await prover_get_claim_offers(
            wallet_handle, json.dumps({"schema_seq_no": schema_seq_no_2})))

    assert len(claim_offers) == 2
    assert {'issuer_did': issuer_did, 'schema_seq_no': schema_seq_no_2} in claim_offers
    assert {'issuer_did': prover_did, 'schema_seq_no': 2} in claim_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_issuer_and_schema(wallet_handle, prepopulated_wallet,
                                                                             issuer_did, claim_offer_issuer_1_json,
                                                                             schema_seq_no):
    claim_offers = json.loads(
        await prover_get_claim_offers(
            wallet_handle, claim_offer_issuer_1_json))

    assert len(claim_offers) == 1
    assert {'issuer_did': issuer_did, 'schema_seq_no': schema_seq_no} in claim_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_no_results(wallet_handle, prepopulated_wallet, schema_seq_no):
    claim_offers = json.loads(
        await prover_get_claim_offers(
            wallet_handle, json.dumps({"schema_seq_no": schema_seq_no + 100})))

    assert len(claim_offers) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet,
                                                                       schema_seq_no):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claim_offers(invalid_wallet_handle, json.dumps({"schema_seq_no": schema_seq_no}))

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
