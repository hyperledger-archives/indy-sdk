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
async def test_build_get_attrib_request_works():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "Th7MpTaRZVRYnPiabds81Y"
    raw = "endpoint"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "104",
            "dest": destination,
            "raw": raw
        }
    }

    response = json.loads((await ledger.build_get_attrib_request(identifier, destination, raw)).decode())
    assert expected_response.items() <= response.items()
