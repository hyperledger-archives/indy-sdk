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
async def test_prover_get_claim_offers_works_for_filter_by_issuer(wallet_handle, prepopulated_wallet, issuer_did,
                                                                  schema_key, xyz_schema_key):
    claim_offers = json.loads(
        await prover_get_claim_offers(wallet_handle, json.dumps({"issuer_did": issuer_did})))

    assert len(claim_offers) == 2
    claim_offers = claim_offers_info(claim_offers)
    assert {"issuer_did": issuer_did, "schema_key": schema_key} in claim_offers
    assert {"issuer_did": issuer_did, "schema_key": xyz_schema_key} in claim_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_schema(wallet_handle, prepopulated_wallet, issuer_did,
                                                                  prover_did, xyz_schema_key):
    claim_offers = json.loads(
        await prover_get_claim_offers(
            wallet_handle, json.dumps({"schema_key": {"name": "xyz"}})))

    assert len(claim_offers) == 1
    claim_offers = claim_offers_info(claim_offers)
    assert {'issuer_did': issuer_did, 'schema_key': xyz_schema_key} in claim_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_part_of_schema(wallet_handle, prepopulated_wallet,
                                                                          issuer_did, prover_did, xyz_schema_key):
    claim_offers = json.loads(
        await prover_get_claim_offers(
            wallet_handle, json.dumps({"schema_key": xyz_schema_key})))

    assert len(claim_offers) == 1
    claim_offers = claim_offers_info(claim_offers)
    assert {'issuer_did': issuer_did, 'schema_key': xyz_schema_key} in claim_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_filter_by_issuer_and_schema(wallet_handle, prepopulated_wallet,
                                                                             issuer_did, schema_key,
                                                                             claim_offer_issuer_1_schema_1_json):
    claim_offers = json.loads(
        await prover_get_claim_offers(wallet_handle, claim_offer_issuer_1_schema_1_json))

    assert len(claim_offers) == 1
    claim_offers = claim_offers_info(claim_offers)
    assert {'issuer_did': issuer_did, 'schema_key': schema_key} in claim_offers


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_no_results(wallet_handle, prepopulated_wallet, schema_key, issuer_did):
    claim_offers = json.loads(
        await prover_get_claim_offers( wallet_handle, json.dumps({"issuer_did": issuer_did + 'a'})))

    assert len(claim_offers) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claim_offers_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet, schema_key):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claim_offers(invalid_wallet_handle, json.dumps({"schema_key": schema_key}))

    assert ErrorCode.WalletInvalidHandle == e.value.error_code


def claim_offers_info(claim_offers):
    return [{"issuer_did": claim_offer['issuer_did'], "schema_key": claim_offer['schema_key']}
            for claim_offer in claim_offers]
