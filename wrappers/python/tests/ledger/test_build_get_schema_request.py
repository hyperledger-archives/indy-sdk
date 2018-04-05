from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_schema_requests_works_for_correct_data_json(did_trustee):
    id_ = "1"

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "3",
            "data": int(id_)
        }
    }

    response = json.loads(await ledger.build_get_schema_request(did_trustee, id_))
    assert expected_response.items() <= response.items()
