from indy import ledger

import json
import pytest


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
        "operation": {"ref": 1,
                      "data": {
                          "primary": {
                              "n": "1", "s": "2", "rms": "3", "r": {"name": "1"}, "rctxt": "1", "z": "1"
                          },
                          "revocation": {}
                      },
                      "type": "102",
                      "signature_type": "CL"}}

    response = json.loads(
        await ledger.build_claim_def_txn(
            did_trustee, schema_seq_no, signature_type, json.dumps(data)))
    assert expected_response.items() <= response.items()
