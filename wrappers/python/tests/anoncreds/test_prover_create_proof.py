from indy import wallet
from indy.anoncreds import prover_get_claims_for_proof_req, prover_create_proof

from tests.utils import storage, anoncreds
from tests.utils.wallet import create_and_open_wallet

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.fixture
async def wallet_handle_and_claim_def():
    handle = await create_and_open_wallet()
    claim_def = await anoncreds.prepare_common_wallet(handle)
    yield (handle, claim_def)
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_prover_create_proof_works(wallet_handle_and_claim_def):
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

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle_and_claim_def[0], json.dumps(proof_req)))
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

    print("requested_claims = {}".format(json.dumps(requested_claims)))

    schemas = {
        claim_for_attr: anoncreds.get_gvt_schema_json(1)
    }

    claim_defs = {
        claim_for_attr: json.loads(wallet_handle_and_claim_def[1])
    }

    await prover_create_proof(wallet_handle_and_claim_def[0], json.dumps(proof_req), json.dumps(requested_claims),
                              json.dumps(schemas), anoncreds.COMMON_MASTER_SECRET_NAME,
                              json.dumps(claim_defs), "{}")
