from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_nym_request_works(cleanup_storage):
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
