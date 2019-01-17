from indy.anoncreds import prover_search_credentials, prover_fetch_credentials, prover_close_credentials_search

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_search_credentials_works_for_empty_filter(wallet_handle, prepopulated_wallet):
    (search_handle, total_count) = await prover_search_credentials(wallet_handle, "{}")

    assert total_count == 3

    credentials = json.loads(
        await prover_fetch_credentials(search_handle, total_count))

    assert len(credentials) == 3

    await prover_close_credentials_search(search_handle)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_search_credentials_works_for_filter_by_issuer_did(wallet_handle, prepopulated_wallet, issuer_did):
    (search_handle, total_count) = \
        await prover_search_credentials(wallet_handle, json.dumps({"issuer_did": issuer_did}))

    assert total_count == 2

    credentials = json.loads(
        await prover_fetch_credentials(search_handle, total_count))

    assert len(credentials) == 2

    await prover_close_credentials_search(search_handle)
