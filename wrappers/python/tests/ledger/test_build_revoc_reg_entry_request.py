from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_revoc_reg_entry_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    rev_reg_entry_value = {
        "accum": "123456789",
        "prevAccum": "123456789",
        "issued": [],
        "revoked": []
    }
    rev_reg_def_id = "RevocRegID"
    rev_reg_type = "CL_ACCUM"

    expected_response = {
        "operation": {
            "type": "114",
            "revocRegDefId": rev_reg_def_id,
            "revocDefType": rev_reg_type,
            "value": rev_reg_entry_value
        }
    }

    request = json.loads(await ledger.build_revoc_reg_entry_request(identifier, rev_reg_def_id, rev_reg_type,
                                                                    json.dumps(rev_reg_entry_value)))
    assert expected_response.items() <= request.items()
