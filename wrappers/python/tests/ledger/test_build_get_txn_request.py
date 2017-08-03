from indysdk import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_txn_request_works(cleanup_storage):
    identifier = "identifier"
    data = 1
    expected_response = {
        "identifier": "identifier",
        "operation": {
            "type": "3", "data": 1
        }
    }

    response = json.loads(await ledger.build_get_txn_request(identifier, data))
    assert expected_response.items() <= response.items()
