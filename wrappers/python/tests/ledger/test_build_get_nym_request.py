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
async def test_build_get_nym_request_works():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "105",
            "dest": destination
        }
    }

    response = json.loads(await ledger.build_get_nym_request(identifier, destination))
    assert expected_response.items() <= response.items()
