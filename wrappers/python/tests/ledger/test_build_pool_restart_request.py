from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_pool_restart_request_work_for_start_action():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "118",
            "action": "start",
            "schedule": {},
        }
    }

    request = json.loads(
        await ledger.build_pool_restart_request(identifier, 'start', '{}'))
    assert expected_response.items() <= request.items()


@pytest.mark.asyncio
async def test_build_pool_restart_request_work_for_cancel_action():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "118",
            "action": "cancel",
        }
    }

    request = json.loads(
        await ledger.build_pool_restart_request(identifier, 'cancel', None))
    assert expected_response.items() <= request.items()
