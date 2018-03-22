from indy.anoncreds import prover_create_proof
from indy.error import ErrorCode, IndyError

import json
import pytest


@pytest.mark.asyncio
async def test_prover_create_proof_works(wallet_handle, prepopulated_wallet, gvt_schema_id, gvt_schema,
                                         master_secret_id, proof_req, id_credential_1, issuer_1_gvt_cred_def_id):
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

    await prover_create_proof(wallet_handle, json.dumps(proof_req), json.dumps(requested_credentials),
                              master_secret_id, json.dumps(schemas), json.dumps(credential_defs), "{}")


@pytest.mark.asyncio
async def test_prover_create_proof_works_for_using_not_satisfy_credential(wallet_handle, prepopulated_wallet,
                                                                          gvt_schema_id, gvt_schema,
                                                                          master_secret_id, id_credential_1,
                                                                          issuer_1_gvt_cred_def_id):
    credential_def_json, _, _, _, _ = prepopulated_wallet

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

    requested_credentials = {
        "self_attested_attributes": {},
        "requested_attributes": {
            "attr1_referent": {"cred_id": id_credential_1, "revealed": True}
        },
        "requested_predicates": {
        }
    }

    schemas = {
        gvt_schema_id: gvt_schema
    }

    credential_defs = {
        issuer_1_gvt_cred_def_id: json.loads(credential_def_json)
    }

    with pytest.raises(IndyError) as e:
        await prover_create_proof(wallet_handle, json.dumps(proof_req), json.dumps(requested_credentials),
                                  master_secret_id, json.dumps(schemas), json.dumps(credential_defs), "{}")

    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_prover_create_proof_works_for_invalid_wallet_handle(wallet_handle, prepopulated_wallet, gvt_schema_id,
                                                                   gvt_schema, master_secret_id, proof_req,
                                                                   id_credential_1, issuer_1_gvt_cred_def_id):
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

    invalid_wallet_handle = wallet_handle + 100

    with pytest.raises(IndyError) as e:
        await prover_create_proof(invalid_wallet_handle, json.dumps(proof_req), json.dumps(requested_credentials),
                                  master_secret_id, json.dumps(schemas), json.dumps(credential_defs), "{}")

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
