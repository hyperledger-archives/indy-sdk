from indy import ledger

import json
import pytest

@pytest.mark.asyncio
async def test_build_get_frozen_ledgers_request(did_trustee):
    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "10"
        }
    }
    response = json.loads(await ledger.build_get_frozen_ledgers_request(did_trustee))
    assert expected_response.items() <= response.items()
