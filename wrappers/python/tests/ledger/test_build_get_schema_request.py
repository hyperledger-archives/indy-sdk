from tests.utils import storage
from indy import ledger

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_build_get_schema_requests_works_for_correct_data_json():
    identifier = "identifier"
    data = '{"name":"name","version":"1.0"}'

    expected_response = {
        "identifier": "identifier",
        "operation": {
            "type": "107",
            "dest": "identifier",
            "data": {
                "name": "name",
                "version": "1.0"
            }
        }
    }

    response = json.loads(await ledger.build_get_schema_request(identifier, identifier, data))
    assert expected_response.items() <= response.items()
