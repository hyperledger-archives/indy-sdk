from indy import ledger
from indy.error import *

import json
import pytest


@pytest.mark.asyncio
async def test_build_pool_config_request_work():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "111",
            "writes": True,
            "force": False,
        }
    }

    request = json.loads(await ledger.build_pool_config_request(identifier, True, False))
    assert expected_response.items() <= request.items()