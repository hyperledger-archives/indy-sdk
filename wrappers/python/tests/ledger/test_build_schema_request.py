from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_schema_requests_works_for_correct_data_json(did_trustee):
    data = '{"id":"1","name":"name","version":"1.0","attrNames":["male"],"ver":"1.0"}'

    expected_response = {
        "operation": {
            "type": "101",
            "data": {"name": "name", "version": "1.0", "attr_names": ["male"]}
        }
    }

    response = json.loads(await ledger.build_schema_request(did_trustee, data))
    assert expected_response.items() <= response.items()
