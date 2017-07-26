from indy.anoncreds import prover_get_claims_for_proof_req, prover_create_proof, prover_get_claims
from indy.error import ErrorCode, IndyError

from tests.utils import anoncreds

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.mark.asyncio
async def test_prover_create_proof_works(init_common_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_uuid": {
                "schema_seq_no": 1,
                "name": "name"
            }
        },
        "requested_predicates": {
            "predicate1_uuid": {
                "attr_name": "age",
                "p_type": "GE",
                "value": 18
            }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(init_common_wallet[0], json.dumps(proof_req)))
    claim_for_attr = claims['attrs']['attr1_uuid'][0]['claim_uuid']
    claim_for_predicate = claims['predicates']['predicate1_uuid'][0]['claim_uuid']
    requested_claims = {
        "self_attested_attributes": {},
        "requested_attrs": {
            "attr1_uuid": [claim_for_attr, True]
        },
        "requested_predicates": {
            "predicate1_uuid": claim_for_predicate
        }
    }

    schemas = {
        claim_for_attr: anoncreds.get_gvt_schema_json(1)
    }

    claim_defs = {
        claim_for_attr: json.loads(init_common_wallet[1])
    }

    await prover_create_proof(init_common_wallet[0], json.dumps(proof_req), json.dumps(requested_claims),
                              json.dumps(schemas), anoncreds.COMMON_MASTER_SECRET_NAME,
                              json.dumps(claim_defs), "{}")


@pytest.mark.asyncio
async def test_prover_create_proof_works_for_using_not_satisfy_claim(init_common_wallet):
    claims = json.loads(await prover_get_claims(init_common_wallet[0], "{}"))
    claim_uuid = claims[0]['claim_uuid']
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_uuid": {
                "schema_seq_no": 1,
                "name": "some_attr"
            }
        },
        "requested_predicates": {}
    }

    requested_claims = {
        "self_attested_attributes": {},
        "requested_attrs": {
            "attr1_uuid": [claim_uuid, True]
        },
        "requested_predicates": {
            "predicate1_uuid": {}
        }
    }

    schemas = {
        claim_uuid: anoncreds.get_gvt_schema_json(1)
    }

    claim_defs = {
        claim_uuid: json.loads(init_common_wallet[1])
    }

    with pytest.raises(IndyError) as e:
        await prover_create_proof(init_common_wallet[0], json.dumps(proof_req), json.dumps(requested_claims),
                                  json.dumps(schemas), anoncreds.COMMON_MASTER_SECRET_NAME,
                                  json.dumps(claim_defs), "{}")
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_prover_create_proof_works_for_invalid_wallet_handle(init_common_wallet):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_uuid": {
                "schema_seq_no": 1,
                "name": "name"
            }
        },
        "requested_predicates": {
            "predicate1_uuid": {
                "attr_name": "age",
                "p_type": "GE",
                "value": 18
            }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(init_common_wallet[0], json.dumps(proof_req)))
    claim_for_attr = claims['attrs']['attr1_uuid'][0]['claim_uuid']
    claim_for_predicate = claims['predicates']['predicate1_uuid'][0]['claim_uuid']
    requested_claims = {
        "self_attested_attributes": {},
        "requested_attrs": {
            "attr1_uuid": [claim_for_attr, True]
        },
        "requested_predicates": {
            "predicate1_uuid": claim_for_predicate
        }
    }

    schemas = {
        claim_for_attr: anoncreds.get_gvt_schema_json(1)
    }

    claim_defs = {
        claim_for_attr: json.loads(init_common_wallet[1])
    }

    invalid_wallet_handle = init_common_wallet[0] + 100

    with pytest.raises(IndyError) as e:
        await prover_create_proof(invalid_wallet_handle, json.dumps(proof_req), json.dumps(requested_claims),
                                  json.dumps(schemas), anoncreds.COMMON_MASTER_SECRET_NAME,
                                  json.dumps(claim_defs), "{}")
    assert ErrorCode.WalletInvalidHandle == e.value.error_code
