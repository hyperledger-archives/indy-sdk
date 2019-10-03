from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_revoc_reg_def_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    rev_reg_def_id = "NcYxiDXkpYi6ov5FcYDi1e:4:NcYxiDXkpYi6ov5FcYDi1e:3:CL:1:CL_ACCUM:TAG_1"

    expected_response = {
        "operation": {
            "type": "115",
            "id": rev_reg_def_id
        }
    }

    request = json.loads(await ledger.build_get_revoc_reg_def_request(identifier, rev_reg_def_id))
    assert expected_response.items() <= request.items()
