from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_claim_def_request_works(did_trustee):
    id_ = did_trustee + ":\u0003:CL:1"

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "108",
            "ref": 1,
            "signature_type": "CL",
            "origin": did_trustee
        }
    }

    response = json.loads((await ledger.build_get_claim_def_txn(did_trustee, id_)))
    assert expected_response.items() <= response.items()