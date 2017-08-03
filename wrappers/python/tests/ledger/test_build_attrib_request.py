from indy_sdk import ledger
from indy_sdk.error import *

import json
import pytest


@pytest.mark.asyncio
async def test_build_attrib_request_works_for_raw_data(cleanup_storage):
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "Th7MpTaRZVRYnPiabds81Y"
    raw = '{"endpoint":{"ha":"127.0.0.1:5555"}}'

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "100",
            "dest": destination,
            "raw": raw
        }
    }

    response = json.loads(await ledger.build_attrib_request(identifier, destination, None, raw, None))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_attrib_request_works_for_missed_attribute(cleanup_storage):
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "Th7MpTaRZVRYnPiabds81Y"

    with pytest.raises(IndyError) as e:
        await ledger.build_attrib_request(identifier, destination, None, None, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


