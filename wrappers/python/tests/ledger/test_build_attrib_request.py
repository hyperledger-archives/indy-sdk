from tests.utils import storage
from indy import ledger
from indy.error import *

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
async def test_build_attrib_request_works_for_raw_data():
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

    response = json.loads((await ledger.build_attrib_request(identifier, destination, None, raw, None)).decode())
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_attrib_request_works_for_missed_attribute():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "Th7MpTaRZVRYnPiabds81Y"

    try:
        await ledger.build_attrib_request(identifier, destination, None, None, None)
    except Exception as e:
        assert type(IndyError(ErrorCode.CommonInvalidStructure)) == type(e) and \
               IndyError(ErrorCode.CommonInvalidStructure).args == e.args

