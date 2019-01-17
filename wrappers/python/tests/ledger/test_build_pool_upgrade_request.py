from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_pool_upgrade_request_work_for_start_action():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "109",
            "name": "upgrade-python",
            "version": "2.0.0",
            "action": "start",
            "sha256": "abc12345",
            "schedule": {},
            "reinstall": False,
            "force": False,
        }
    }

    request = json.loads(
        await ledger.build_pool_upgrade_request(identifier, 'upgrade-python', '2.0.0', 'start', 'abc12345',
                                                None, '{}', None, False, False, None))
    assert expected_response.items() <= request.items()


@pytest.mark.asyncio
async def test_build_pool_upgrade_request_work_for_cancel_action():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "109",
            "name": "upgrade-python",
            "version": "2.0.0",
            "action": "cancel",
            "sha256": "abc12345",
            "reinstall": False,
            "force": False,
        }
    }

    request = json.loads(
        await ledger.build_pool_upgrade_request(identifier, 'upgrade-python', '2.0.0', 'cancel', 'abc12345',
                                                None, None, None, False, False, None))
    assert expected_response.items() <= request.items()


@pytest.mark.asyncio
async def test_build_pool_upgrade_request_work_for_package():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "109",
            "name": "upgrade-python",
            "version": "2.0.0",
            "action": "start",
            "sha256": "abc12345",
            "schedule": {},
            "reinstall": False,
            "force": False,
            "package": "some_package"
        }
    }

    request = json.loads(
        await ledger.build_pool_upgrade_request(identifier, 'upgrade-python', '2.0.0', 'start', 'abc12345',
                                                None, '{}', None, False, False, "some_package"))
    assert expected_response.items() <= request.items()
