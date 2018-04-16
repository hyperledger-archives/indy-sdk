from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_revoc_reg_entry_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    rev_reg_entry_value = {
        "ver": "1.0",
        "value": {
            "accum": "false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
        }
    }
    rev_reg_def_id = "RevocRegID"
    rev_reg_type = "CL_ACCUM"

    expected_response = {
        "operation": {
            "type": "114",
            "revocRegDefId": "RevocRegID",
            "revocDefType": "CL_ACCUM",
            "value": {"accum": "false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"}
        }
    }

    request = json.loads(await ledger.build_revoc_reg_entry_request(identifier, rev_reg_def_id, rev_reg_type,
                                                                    json.dumps(rev_reg_entry_value)))
    assert expected_response.items() <= request.items()
