from indy import wallet
from indy.anoncreds import prover_get_claims_for_proof_req
from indy.error import ErrorCode, IndyError

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
async def wallet_handle():
    handle = await create_and_open_wallet()
    await anoncreds.prepare_common_wallet(handle)
    yield handle
    await wallet.close_wallet(handle)


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_revealed_attr(wallet_handle):
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
        "requested_predicates": {}
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))
    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_uuid']) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_not_found_attribute(wallet_handle):
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

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))
    assert len(claims['attrs']) == 1
    assert len(claims['predicates']) == 0
    assert len(claims['attrs']['attr1_uuid']) == 0


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_satisfy_predicate(wallet_handle):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_uuid":
                {
                    "attr_name": "age",
                    "p_type": "GE",
                    "value": 18
                }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))
    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_uuid']) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_not_satisfy_predicate(wallet_handle):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_uuid":
                {
                    "attr_name": "age",
                    "p_type": "GE",
                    "value": 58
                }
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))
    assert len(claims['attrs']) == 0
    assert len(claims['predicates']) == 1
    assert len(claims['predicates']['predicate1_uuid']) == 0


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_multiply_attribute_and_predicates(wallet_handle):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {
            "attr1_uuid": {"schema_seq_no": 1, "name": "name"},
            "attr2_uuid": {"schema_seq_no": 1, "name": "sex"}
        },
        "requested_predicates": {
            "predicate1_uuid": {"attr_name": "age", "p_type": "GE", "value": 18},
            "predicate2_uuid": {"attr_name": "height", "p_type": "GE", "value": 160}
        }
    }

    claims = json.loads(await prover_get_claims_for_proof_req(wallet_handle, json.dumps(proof_req)))
    assert len(claims['attrs']) == 2
    assert len(claims['predicates']) == 2
    assert len(claims['attrs']['attr1_uuid']) == 1
    assert len(claims['attrs']['attr2_uuid']) == 1
    assert len(claims['predicates']['predicate1_uuid']) == 1
    assert len(claims['predicates']['predicate2_uuid']) == 1


@pytest.mark.asyncio
async def test_prover_get_claims_for_proof_req_works_for_invalid_wallet_handle(wallet_handle):
    proof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attrs": {},
        "requested_predicates": {
            "predicate1_uuid":
                {
                    "attr_name": "age",
                    "p_type": "GE",
                    "value": 58
                }
        }
    }

    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_get_claims_for_proof_req(invalid_wallet_handle, json.dumps(proof_req))
    assert ErrorCode.WalletInvalidHandle == e.value.error_code


