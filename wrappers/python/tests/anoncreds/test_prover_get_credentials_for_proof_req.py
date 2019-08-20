import json
import pytest

from indy.anoncreds import prover_get_credentials_for_proof_req


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr(wallet_handle, prepopulated_wallet):
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

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 1
    assert len(credentials['predicates']) == 0
    assert len(credentials['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_for_specific_schema(wallet_handle,
                                                                                                prepopulated_wallet,
                                                                                                gvt_schema_id):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"schema_id": gvt_schema_id}]
            }
        },
        "requested_predicates": {}
    }

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 1
    assert len(credentials['predicates']) == 0
    assert len(credentials['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_not_found_attribute(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "some_attr"
            }
        },
        "requested_predicates": {}
    }

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 1
    assert len(credentials['predicates']) == 0
    assert len(credentials['attrs']['attr1_referent']) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_predicate(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "name": "age",
                    "p_type": ">=",
                    "p_value": 18
                }
        }
    }

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 0
    assert len(credentials['predicates']) == 1
    assert len(credentials['predicates']['predicate1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_predicate_for_schema_id(wallet_handle, gvt_schema_id,
                                                                                      prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "name": "age",
                    "p_type": ">=",
                    "p_value": 18,
                    "restrictions": [{"schema_id": gvt_schema_id}]
                }
        }
    }

    credentials = json.loads(await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 0
    assert len(credentials['predicates']) == 1
    assert len(credentials['predicates']['predicate1_referent']) == 2


@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_not_found_predicate_attribute(wallet_handle,
                                                                                            prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "name": "some_attr",
                    "p_type": ">=",
                    "p_value": 58
                }
        }
    }

    credentials = json.loads(await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 0
    assert len(credentials['predicates']) == 1
    assert len(credentials['predicates']['predicate1_referent']) == 0


@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_not_satisfy_predicate(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "name": "age",
                    "p_type": ">=",
                    "p_value": 58
                }
        }
    }

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 0
    assert len(credentials['predicates']) == 1
    assert len(credentials['predicates']['predicate1_referent']) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_multiply_attribute_and_predicates(wallet_handle,
                                                                                                prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {"name": "name"},
            "attr2_referent": {"name": "sex"}
        },
        "requested_predicates": {
            "predicate1_referent": {"name": "age", "p_type": ">=", "p_value": 18},
            "predicate2_referent": {"name": "height", "p_type": ">=", "p_value": 160}
        }
    }

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 2
    assert len(credentials['predicates']) == 2
    assert len(credentials['attrs']['attr1_referent']) == 2
    assert len(credentials['attrs']['attr2_referent']) == 2
    assert len(credentials['predicates']['predicate1_referent']) == 2
    assert len(credentials['predicates']['predicate2_referent']) == 2
