from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_cred_def_request_works(did_trustee):
    id_ = did_trustee + ":3:CL:1:TAG_1"

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "108",
            "ref": 1,
            "signature_type": "CL",
            "origin": did_trustee,
            "tag": "TAG_1"
        }
    }

    response = json.loads((await ledger.build_get_cred_def_request(did_trustee, id_)))
    assert expected_response.items() <= response.items()