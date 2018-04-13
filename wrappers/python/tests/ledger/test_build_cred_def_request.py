from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_cred_def_request_works_for_correct_data_json(did_trustee):
    data = {
        "ver": "1.0",
        "id": "cred_def_id",
        "schemaId": "1",
        "type": "CL",
        "tag": "TAG_1",
        "value": {
            "primary": {
                "n": "1",
                "s": "2",
                "rms": "3",
                "r": {"name": "1"},
                "rctxt": "1",
                "z": "1"
            }
        }
    }

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "ref": 1,
            "data": {
                "primary": {"n": "1", "s": "2", "rms": "3", "r": {"name": "1"}, "rctxt": "1", "z": "1"}
            },
            "type": "102",
            "signature_type": "CL"
        }
    }

    response = json.loads(await ledger.build_cred_def_request(did_trustee, json.dumps(data)))
    assert expected_response.items() <= response.items()
