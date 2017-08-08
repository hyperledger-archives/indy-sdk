from indy_sdk.anoncreds import prover_get_claims
from indy_sdk.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_filter(wallet_handle, prepopulated_wallet):
    claims = json.loads(
        await prover_get_claims(wallet_handle, "{}"))

    assert len(claims) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did(wallet_handle, prepopulated_wallet, issuer_did):
    claims = json.loads(
        await prover_get_claims(wallet_handle, json.dumps({"issuer_did": issuer_did})))

    assert len(claims) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_filter_by_issuer_did_and_schema_seq_no(wallet_handle, prepopulated_wallet,
                                                                                  claim_offer_issuer_1_json):
    claims = json.loads(
        await prover_get_claims(wallet_handle, claim_offer_issuer_1_json))

    assert len(claims) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_empty_result(wallet_handle, prepopulated_wallet, schema_seq_no):
    claims = json.loads(
        await prover_get_claims(
            wallet_handle, json.dumps({"schema_seq_no": schema_seq_no + 100})))

    assert len(claims) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claims(invalid_wallet_handle, '{}')

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
