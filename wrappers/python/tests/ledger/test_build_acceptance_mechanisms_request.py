from indy import ledger

import json
import pytest


aml = {
    'acceptance mechanism label 1': 'some acceptance mechanism description 1'
}
version = '1.0.0'


@pytest.mark.asyncio
async def test_build_acceptance_mechanisms_request():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "5",
            "aml": aml,
            "version": version,
        }
    }

    response = json.loads(await ledger.build_acceptance_mechanisms_request(identifier, json.dumps(aml), version, None))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_acceptance_mechanisms_request_with_context():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    aml_context = "some context"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "5",
            "aml": aml,
            "version": version,
            "amlContext": aml_context,
        }
    }

    response = json.loads(
        await ledger.build_acceptance_mechanisms_request(identifier, json.dumps(aml), version, aml_context))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_get_acceptance_mechanisms_request():
    expected_response = {
        "operation": {
            "type": "7"
        }
    }

    response = json.loads(await ledger.build_get_acceptance_mechanisms_request(None, None, None))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_get_acceptance_mechanisms_request_for_timestamp():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    timestamp = 123456789

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "7",
            "timestamp": timestamp,
        }
    }

    response = json.loads(await ledger.build_get_acceptance_mechanisms_request(identifier, timestamp, None))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_get_acceptance_mechanisms_request_for_version():
    identifier = "Th7MpTaRZVRYnPiabds81Y"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "7",
            "version": version,
        }
    }

    response = json.loads(await ledger.build_get_acceptance_mechanisms_request(identifier, None, version))
    assert expected_response.items() <= response.items()
