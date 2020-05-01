import json
import pytest

from indy.anoncreds import generate_nonce, prover_create_proof, verifier_verify_proof
from indy import error


@pytest.mark.asyncio
async def test_proof_works_for_valid_predicates(
    wallet_handle,
    prepopulated_wallet,
    gvt_schema_id,
    gvt_schema,
    master_secret_id,
    id_credential_1,
    issuer_1_gvt_cred_def_id
):
    credential_def_json, _, _, _, _ = prepopulated_wallet

    requested_credentials = {
        "self_attested_attributes": {},
        "requested_attributes": {
            "attr1_referent": {"cred_id": id_credential_1, "revealed": True}
        },
        "requested_predicates": {
            "predicate1_referent": {"cred_id": id_credential_1}
        }
    }

    schemas = {
        gvt_schema_id: gvt_schema
    }

    credential_defs = {
        issuer_1_gvt_cred_def_id: json.loads(credential_def_json)
    }

    criteria = {
        '<': 88,
        '<=': 88,
        '>=': 18,
        '>': 18
    }
    for (pred, threshold) in criteria.items():
        proof_req = {
            "nonce": await generate_nonce(),
            "name": "proof_req_1[age {} {}]".format(pred, threshold),
            "version": "0.1",
            "requested_attributes": {
                "attr1_referent": {"name": "name"}
            },
            "requested_predicates": {
                "predicate1_referent": {
                    "name": "age",
                    "p_type": pred,
                    "p_value": threshold
                }
            }
        }

        proof_json = await prover_create_proof(
            wallet_handle,
            json.dumps(proof_req),
            json.dumps(requested_credentials),
            master_secret_id,
            json.dumps(schemas),
            json.dumps(credential_defs),
            "{}"
        )

        valid = await verifier_verify_proof(
            json.dumps(proof_req),
            proof_json,
            json.dumps(schemas),
            json.dumps(credential_defs),
            "{}",
            "{}"
        )
        assert valid

@pytest.mark.asyncio
async def test_proof_works_for_bad_predicate(
    wallet_handle,
    prepopulated_wallet,
    gvt_schema_id,
    gvt_schema,
    master_secret_id,
    id_credential_1,
    issuer_1_gvt_cred_def_id
):
    credential_def_json, _, _, _, _ = prepopulated_wallet

    requested_credentials = {
        "self_attested_attributes": {},
        "requested_attributes": {
            "attr1_referent": {"cred_id": id_credential_1, "revealed": True}
        },
        "requested_predicates": {
            "predicate1_referent": {"cred_id": id_credential_1}
        }
    }

    schemas = {
        gvt_schema_id: gvt_schema
    }

    credential_defs = {
        issuer_1_gvt_cred_def_id: json.loads(credential_def_json)
    }

    criteria = {
        '==': 28,
        'EQ': 28,
        '!=': 48,
        'NE': 48,
        'LT': 88,
        'LE': 88,
        'GE': 18,
        'GT': 18
    }
    for (pred, threshold) in criteria.items():
        proof_req = {
            "nonce": await generate_nonce(),
            "name": "proof_req_1[age {} {}]".format(pred, threshold),
            "version": "0.1",
            "requested_attributes": {
                "attr1_referent": {"name": "name"}
            },
            "requested_predicates": {
                "predicate1_referent": {
                    "name": "age",
                    "p_type": pred,
                    "p_value": threshold
                }
            }
        }

        with pytest.raises(error.CommonInvalidStructure):
            await prover_create_proof(
                wallet_handle,
                json.dumps(proof_req),
                json.dumps(requested_credentials),
                master_secret_id,
                json.dumps(schemas),
                json.dumps(credential_defs),
                "{}"
            )
