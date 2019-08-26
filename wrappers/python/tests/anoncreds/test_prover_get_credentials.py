import json
import pytest

from indy.anoncreds import prover_get_credentials


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_empty_filter(wallet_handle, prepopulated_wallet):
    credentials = json.loads(
        await prover_get_credentials(wallet_handle, "{}"))

    assert len(credentials) == 3


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_filter_by_schema_id(wallet_handle, prepopulated_wallet, gvt_schema_id):
    credentials = json.loads(
        await prover_get_credentials(wallet_handle, json.dumps({"schema_id": gvt_schema_id})))

    assert len(credentials) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_works_for_empty_result(wallet_handle, prepopulated_wallet, issuer_did):
    credentials = json.loads(
        await prover_get_credentials(
            wallet_handle, json.dumps({"issuer_did": issuer_did + 'a'})))

    assert len(credentials) == 0
