from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_schema_requests_works_for_correct_data_json(did_trustee):
    data = '{"name":"name", "version":"1.0", "attr_names":["name","male"]}'

    expected_response = {
        "operation": {
            "type": "101",
            "data": {"name":"name", "version":"1.0", "attr_names":["name","male"]}
        }
    }

    response = json.loads(await ledger.build_schema_request(did_trustee, data))
    assert expected_response.items() <= response.items()
