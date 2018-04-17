from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_revoc_reg_def_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    rev_reg_def_id = "RevocRegID"

    expected_response = {
        "operation": {
            "type": "115",
            "id": rev_reg_def_id
        }
    }

    request = json.loads(await ledger.build_get_revoc_reg_def_request(identifier, rev_reg_def_id))
    assert expected_response.items() <= request.items()
