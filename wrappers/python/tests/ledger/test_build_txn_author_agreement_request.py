from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_txn_author_agreement_request():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    text = "indy agreement"
    version = "1.0.0"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "4",
            "text": text,
            "version": version
        }
    }

    response = json.loads(await ledger.build_txn_author_agreement_request(identifier, text, version))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_get_txn_author_agreement_request():
    expected_response = {
        "operation": {
            "type": "6",
        }
    }

    response = json.loads(await ledger.build_get_txn_author_agreement_request(None, None))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_get_txn_author_agreement_request_for_hash():
    data = {
        'digest': '83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3'
    }

    expected_response = {
        "operation": {
            "type": "6",
            "digest": data['digest'],
        }
    }

    response = json.loads(await ledger.build_get_txn_author_agreement_request(None, json.dumps(data)))
    assert expected_response.items() <= response.items()
