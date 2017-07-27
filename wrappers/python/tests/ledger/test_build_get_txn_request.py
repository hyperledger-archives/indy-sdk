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
async def test_build_get_txn_request_works():
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
