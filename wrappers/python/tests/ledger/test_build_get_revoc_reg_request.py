from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_revoc_reg_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    rev_reg_def_id = "NcYxiDXkpYi6ov5FcYDi1e:4:NcYxiDXkpYi6ov5FcYDi1e:3:CL:1:CL_ACCUM:TAG_1"
    timestamp = 100

    expected_response = {
        "operation": {
            "type": "116",
            "revocRegDefId": rev_reg_def_id,
            "timestamp": timestamp
        }
    }

    request = json.loads(await ledger.build_get_revoc_reg_request(identifier, rev_reg_def_id, timestamp))
    assert expected_response.items() <= request.items()
