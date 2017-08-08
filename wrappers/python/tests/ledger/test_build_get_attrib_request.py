from indy_sdk import ledger

import json
import pytest


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

    response = json.loads(await ledger.build_get_attrib_request(identifier, destination, raw))
    assert expected_response.items() <= response.items()
