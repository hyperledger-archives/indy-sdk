from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_revoc_reg_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    rev_reg_def_id = "RevocRegID"
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
