from indy.anoncreds import prover_get_credentials_for_proof_req
from indy.error import ErrorCode, IndyError

import json
import pytest


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
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_in_upper_case(wallet_handle,
                                                                                          prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "NAME"
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
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_contains_spaces(wallet_handle,
                                                                                            prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "    name "
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
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_for_schema_name(wallet_handle,
                                                                                            prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"schema_name": "gvt"}]
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
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_for_issuer_did(wallet_handle,
                                                                                           prepopulated_wallet,
                                                                                           issuer_did):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"issuer_did": issuer_did}]
            }
        },
        "requested_predicates": {}
    }

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 1
    assert len(credentials['predicates']) == 0
    assert len(credentials['attrs']['attr1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_for_cred_def_id(wallet_handle,
                                                                                            prepopulated_wallet,
                                                                                            issuer_1_gvt_cred_def_id):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"cred_def_id": issuer_1_gvt_cred_def_id}]
            }
        },
        "requested_predicates": {}
    }

    credentials = json.loads(
        await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 1
    assert len(credentials['predicates']) == 0
    assert len(credentials['attrs']['attr1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_for_multiple_schemas(wallet_handle,
                                                                                                 prepopulated_wallet,
                                                                                                 gvt_schema_id,
                                                                                                 xyz_schema_id):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"schema_id": gvt_schema_id}, {"schema_id": xyz_schema_id}]
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
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_for_schema_id_or_issuer_did(
        wallet_handle, prepopulated_wallet, gvt_schema_id, issuer_did_2):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": [{"issuer_did": issuer_did_2}, {"schema_id": gvt_schema_id}]
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
async def test_prover_get_credentials_for_proof_req_works_for_revealed_attr_for_other_issuer(wallet_handle,
                                                                                             prepopulated_wallet,
                                                                                             issuer_did_2):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "status",
                "restrictions": [{"issuer_did": issuer_did_2}]
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
async def test_prover_get_credentials_for_proof_req_works_for_predicate_attr_in_upper_case(wallet_handle,
                                                                                           prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "name": "AGE",
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
async def test_prover_get_credentials_for_proof_req_works_for_predicate_attr_contains_spaces(wallet_handle,
                                                                                             prepopulated_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {},
        "requested_predicates": {
            "predicate1_referent":
                {
                    "name": " age ",
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


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_predicate_for_issuer(wallet_handle, issuer_did,
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
                    "restrictions": [{"issuer_did": issuer_did}]
                }
        }
    }

    credentials = json.loads(await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 0
    assert len(credentials['predicates']) == 1
    assert len(credentials['predicates']['predicate1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_predicate_for_cred_def_id(wallet_handle,
                                                                                        issuer_1_gvt_cred_def_id,
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
                    "restrictions": [{"cred_def_id": issuer_1_gvt_cred_def_id}]
                }
        }
    }

    credentials = json.loads(await prover_get_credentials_for_proof_req(wallet_handle, json.dumps(proof_req)))

    assert len(credentials['attrs']) == 0
    assert len(credentials['predicates']) == 1
    assert len(credentials['predicates']['predicate1_referent']) == 1


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_predicate_for_multiple_issuers(wallet_handle,
                                                                                             issuer_did,
                                                                                             issuer_did_2,
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
                    "restrictions": [{"issuer_did": issuer_did}, {"issuer_did": issuer_did_2}]
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


# noinspection PyUnusedLocal
@pytest.mark.asyncio
async def test_prover_get_credentials_for_proof_req_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet):
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

    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_credentials_for_proof_req(invalid_wallet_handle, json.dumps(proof_req))

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
