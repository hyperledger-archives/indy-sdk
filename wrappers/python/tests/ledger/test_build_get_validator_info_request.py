from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_validator_info_request_works(did_trustee):
    data = 1
    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "119"
        }
    }

    response = json.loads(await ledger.build_get_validator_info_request(did_trustee))
    assert expected_response.items() <= response.items()
