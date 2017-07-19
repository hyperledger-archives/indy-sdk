from tests.utils import storage
from indy import ledger
from indy.error import ErrorCode, IndyError

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
async def test_build_nym_request_works_for_invalid_identifier():
    identifier = "invalid_base58_identifier"
    dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"

    try:
        await ledger.build_nym_request(identifier, dest, None, None, None)
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.CommonInvalidStructure)) == type(e) and \
               IndyError(ErrorCode.CommonInvalidStructure).args == e.args


@pytest.mark.asyncio
async def test_build_nym_request_works_for_only_required_fields():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "1",
            "dest": destination
        }
    }

    response = json.loads((await ledger.build_nym_request(identifier, destination, None, None, None)).decode())
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_nym_request_works_with_option_fields():
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"
    ver_key = "Anfh2rjAcxkE249DcdsaQl"
    role = "STEWARD"
    alias = "some_alias"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "1",
            "dest": destination,
            "verkey": ver_key,
            "alias": alias,
            "role": "2"
        }
    }

    response = json.loads((await ledger.build_nym_request(identifier, destination, ver_key, alias, role)).decode())
    assert expected_response.items() <= response.items()