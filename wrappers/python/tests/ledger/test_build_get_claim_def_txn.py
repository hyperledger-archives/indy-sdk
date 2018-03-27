from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_claim_def_request_works(did_trustee):
    _ref = 1
    signature_type = "signature_type"

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "108",
            "ref": 1,
            "signature_type": "signature_type",
            "origin": did_trustee
        }
    }

    response = json.loads((await ledger.build_get_claim_def_txn(
        did_trustee, _ref, signature_type, did_trustee
    )))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_claim_def_request_works_for_correct_data_json(did_trustee):
    signature_type = "CL"
    schema_seq_no = 1
    data = {
        "primary": {
            "n": "1",
            "s": "2",
            "rms": "3",
            "r": {
                "name": "1"
            },
            "rctxt": "1",
            "z": "1"
        }
    }

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "ref": schema_seq_no,
            "data": data,
            "type": "102",
            "signature_type": signature_type
        }
    }

    response = json.loads(
        await ledger.build_claim_def_txn(
            did_trustee, schema_seq_no, signature_type, json.dumps(data)))

    assert expected_response.items() <= response.items()
