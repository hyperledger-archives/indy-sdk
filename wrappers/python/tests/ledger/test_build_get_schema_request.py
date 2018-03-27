from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_schema_requests_works_for_correct_data_json(did_trustee):
    data = '{"name":"name","version":"1.0"}'

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "107",
            "dest": did_trustee,
            "data": {
                "name": "name",
                "version": "1.0"
            }
        }
    }

    response = json.loads(await ledger.build_get_schema_request(did_trustee, did_trustee, data))
    assert expected_response.items() <= response.items()
