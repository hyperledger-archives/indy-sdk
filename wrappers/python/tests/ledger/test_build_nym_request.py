from indy_sdk import ledger
from indy_sdk.error import ErrorCode, IndyError

import json
import pytest


@pytest.mark.asyncio
async def test_build_nym_request_works_for_invalid_identifier(cleanup_storage):
    identifier = "invalid_base58_identifier"
    dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"

    with pytest.raises(IndyError) as e:
        await ledger.build_nym_request(identifier, dest, None, None, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_build_nym_request_works_for_only_required_fields(cleanup_storage):
    identifier = "Th7MpTaRZVRYnPiabds81Y"
    destination = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"

    expected_response = {
        "identifier": identifier,
        "operation": {
            "type": "1",
            "dest": destination
        }
    }

    response = json.loads((await ledger.build_nym_request(identifier, destination, None, None, None)))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_nym_request_works_with_option_fields(cleanup_storage):
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

    response = json.loads(await ledger.build_nym_request(identifier, destination, ver_key, alias, role))
    assert expected_response.items() <= response.items()
