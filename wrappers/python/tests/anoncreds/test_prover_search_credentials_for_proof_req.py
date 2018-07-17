from indy.anoncreds import prover_search_credentials_for_proof_req, prover_fetch_credentials_for_proof_req, \
    prover_close_credentials_search_for_proof_req

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_search_credentials_for_proof_req_works(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name"
            }
        },
        "requested_predicates": {
            "predicate1_referent":
                {
                    "name": "age",
                    "p_type": ">=",
                    "p_value": 18
                }
        }
    }

    search_handle = await prover_search_credentials_for_proof_req(wallet_handle, json.dumps(proof_req), None)

    credentials = json.loads(
        await prover_fetch_credentials_for_proof_req(search_handle, "attr1_referent", 3))

    assert len(credentials) == 2

    credentials = json.loads(
        await prover_fetch_credentials_for_proof_req(search_handle, "predicate1_referent", 3))

    assert len(credentials) == 2

    await prover_close_credentials_search_for_proof_req(search_handle)


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_search_credentials_for_proof_req_works_for_extra_query(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name"
            }
        },
        "requested_predicates": {}
    }

    extra_query = {
        'attr1_referent': {
            'attr::name::value': 'Alex'
        }
    }

    search_handle = await prover_search_credentials_for_proof_req(wallet_handle, json.dumps(proof_req), None)

    credentials = json.loads(
        await prover_fetch_credentials_for_proof_req(search_handle, "attr1_referent", 3))

    assert len(credentials) == 2

    await prover_close_credentials_search_for_proof_req(search_handle)
