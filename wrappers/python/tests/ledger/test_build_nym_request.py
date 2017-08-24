from indy import ledger, signus
from indy.error import ErrorCode, IndyError

import json
import pytest


@pytest.mark.asyncio
async def test_build_nym_request_works_for_invalid_identifier():
    identifier = "invalid_base58_identifier"
    dest = "FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4"

    with pytest.raises(IndyError) as e:
        await ledger.build_nym_request(identifier, dest, None, None, None)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


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

    response = json.loads((await ledger.build_nym_request(identifier, destination, None, None, None)))
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

    response = json.loads(await ledger.build_nym_request(identifier, destination, ver_key, alias, role))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_nym_request_works_for_different_roles(wallet_handle, pool_handle, identity_trustee1):
    (trustee_did, _) = identity_trustee1

    await check_for_role(pool_handle, wallet_handle, trustee_did, 'TRUST_ANCHOR', '101')
    await check_for_role(pool_handle, wallet_handle, trustee_did, 'TRUSTEE', '0')
    await check_for_role(pool_handle, wallet_handle, trustee_did, 'STEWARD', '2')


@pytest.mark.asyncio
async def check_for_role(pool_handle, wallet_handle, trustee_did, role, expected_role_value):
    (my_did, my_verkey, _) = await signus.create_and_store_my_did(wallet_handle, "{}")

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_verkey, None, role)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    get_nym_request = await ledger.build_get_nym_request(my_did, my_did)
    get_nym_response = json.loads(await ledger.submit_request(pool_handle, get_nym_request))
    assert expected_role_value == json.loads(get_nym_response['result']['data'])['role']


@pytest.mark.asyncio
async def test_nym_request_works_for_invalid_role(identity_trustee1, identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, _) = identity_my1

    with pytest.raises(IndyError) as e:
        await ledger.build_nym_request(trustee_did, my_did, None, None, "WRONG_ROLE")
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
