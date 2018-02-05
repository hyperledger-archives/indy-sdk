from indy.anoncreds import prover_get_claims
from indy.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_filter(wallet_handle, prepopulated_wallet):
    claims = json.loads(
        await prover_get_claims(wallet_handle, "{}"))

    assert len(claims) == 3


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did(wallet_handle, prepopulated_wallet, issuer_did):
    claims = json.loads(
        await prover_get_claims(wallet_handle, json.dumps({"issuer_did": issuer_did})))

    assert len(claims) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_schema(wallet_handle, prepopulated_wallet, schema_key):
    claims = json.loads(
        await prover_get_claims(wallet_handle, json.dumps({"schema_key": schema_key})))

    assert len(claims) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_part_of_schema(wallet_handle, prepopulated_wallet, schema_key):
    claims = json.loads(
        await prover_get_claims(wallet_handle, json.dumps({"schema_key": {"name": "gvt"}})))

    assert len(claims) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did_and_schema_seq_no(wallet_handle, prepopulated_wallet,
                                                                                  claim_offer_issuer_1_schema_1_json):
    claims = json.loads(
        await prover_get_claims(wallet_handle, claim_offer_issuer_1_schema_1_json))

    assert len(claims) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_result(wallet_handle, prepopulated_wallet, issuer_did):
    claims = json.loads(
        await prover_get_claims(
            wallet_handle, json.dumps({"issuer_did": issuer_did + 'a'})))

    assert len(claims) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claims(invalid_wallet_handle, '{}')

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
