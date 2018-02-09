from indy.anoncreds import prover_get_claims_for_proof_req
from indy.error import ErrorCode, IndyError

import json
import pytest


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name"
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_in_upper_case(wallet_handle,
                                                                                     prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "NAME"
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_contains_spaces(wallet_handle,
                                                                                       prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": " name "
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_schema(wallet_handle,
                                                                                           prepopulated_wallet,
                                                                                           schema_key):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"schema_key": schema_key}]
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_for_part_of_schema(wallet_handle,
                                                                                          prepopulated_wallet,
                                                                                          schema_key):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"schema_key": {"name": "gvt"}}]
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_issuer(wallet_handle,
                                                                                           prepopulated_wallet,
                                                                                           issuer_did):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"issuer_did": issuer_did}]
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_issuer_schema(wallet_handle,
                                                                                                  prepopulated_wallet,
                                                                                                  schema_key,
                                                                                                  issuer_did):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"issuer_did": issuer_did, "schema_key": schema_key}]
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_for_multiple_schemas(wallet_handle,
                                                                                            prepopulated_wallet,
                                                                                            schema_key,
                                                                                            xyz_schema_key):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"schema_key": schema_key}, {"schema_key": xyz_schema_key}]
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_schema_or_specific_issuer(
        wallet_handle, prepopulated_wallet, schema_key, issuer_did):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"issuer_did": issuer_did}, {"schema_key": schema_key}]
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_not_found_attribute(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "some_attr"
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr_for_other_schema_issuer_pair(wallet_handle,
                                                                                                    prepopulated_wallet,
                                                                                                    issuer_did,
                                                                                                    xyz_schema_key):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"issuer_did": issuer_did, "schema_key": xyz_schema_key}]
            }
        },
        "requested_predicates": {}
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_referent']) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_predicate(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "age",
                    "p_type": ">=",
                    "value": 18
                }
        }
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_predicate_attr_in_upper_case(wallet_handle,
                                                                                      prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "AGE",
                    "p_type": ">=",
                    "value": 18
                }
        }
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_predicate_attr_contains_spaces(wallet_handle,
                                                                                        prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": " age ",
                    "p_type": ">=",
                    "value": 18
                }
        }
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_predicate_for_specific_schema(wallet_handle, schema_key,
                                                                                       prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "age",
                    "p_type": ">=",
                    "value": 18,
                    "restrictions": [{"schema_key": schema_key}]
                }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_predicate_for_specific_issuer(wallet_handle, issuer_did,
                                                                                       prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "age",
                    "p_type": ">=",
                    "value": 18,
                    "restrictions": [{"issuer_did": issuer_did}]
                }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_predicate_for_specific_schema_issuer_pair(wallet_handle,
                                                                                                   schema_key,
                                                                                                   issuer_did,
                                                                                                   prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "age",
                    "p_type": ">=",
                    "value": 18,
                    "restrictions": [{"schema_key": schema_key, "issuer_did": issuer_did}]
                }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_predicate_for_multiple_schemas(wallet_handle, schema_key,
                                                                                        xyz_schema_key,
                                                                                        prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "age",
                    "p_type": ">=",
                    "value": 18,
                    "restrictions": [{"schema_key": schema_key}, {"schema_key": xyz_schema_key}]
                }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 2


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_not_found_predicate_attribute(wallet_handle,
                                                                                       prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "some_attr",
                    "p_type": ">=",
                    "value": 58
                }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 0


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_not_satisfy_predicate(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "age",
                    "p_type": ">=",
                    "value": 58
                }
        }
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_referent']) == 0


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_multiply_attribute_and_predicates(wallet_handle,
                                                                                           prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_referent": {"name": "name"},
            "attr2_referent": {"name": "sex"}
        },
        "requested_predicates": {
            "predicate1_referent": {"attr_name": "age", "p_type": ">=", "value": 18},
            "predicate2_referent": {"attr_name": "height", "p_type": ">=", "value": 160}
        }
    }

    claims = json.loads(
        await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(claims['attrs']) == 2
    assert len(claims['predicates']) == 2
    assert len(claims['attrs']['attr1_referent']) == 2
    assert len(claims['attrs']['attr2_referent']) == 2
    assert len(claims['predicates']['predicate1_referent']) == 2
    assert len(claims['predicates']['predicate2_referent']) == 2


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "attr_name": "age",
                    "p_type": ">=",
                    "value": 58
                }
        }
    }

    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claims_for_proof_req(invalid_wallet_handle, json.dumps(proof_req))

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
