from indy import ledger

import json
import pytest

id_ = "V4SGRU86Z58d6TV7PBUe6f:2:name:1.0"


@pytest.mark.asyncio
async def test_build_get_schema_requests_works_for_correct_data_json(did_trustee):
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

    response = json.loads(await ledger.build_get_schema_request(did_trustee, id_))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_get_schema_requests_works_for_default_submitter():
    json.loads(await ledger.build_get_schema_request(None, id_))
