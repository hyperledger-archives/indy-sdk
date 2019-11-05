import datetime
import json
import logging
import time

import pytest

from indy import ledger, anoncreds, blob_storage, error


@pytest.mark.asyncio
async def test_submit_request_works(pool_handle):
    request = {
        "reqId": 1491566332010860,
        "identifier": "Th7MpTaRZVRYnPiabds81Y",
        "operation": {
            "type": "105",
            "dest": "Th7MpTaRZVRYnPiabds81Y"
        },
        "protocolVersion": 2,
        "signature": "4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV"
    }

    expected_response = {
        "result": {
            "reqId": 1491566332010860,
            "identifier": "Th7MpTaRZVRYnPiabds81Y",
            "dest": "Th7MpTaRZVRYnPiabds81Y",
            "data": "{\"dest\":\"Th7MpTaRZVRYnPiabds81Y\",\"identifier\":\"V4SGRU86Z58d6TV7PBUe6f\",\"role\":\"2\""
                    ",\"seqNo\":2,\"txnTime\":null,\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}",
            "type": "105",
        },
        "op": "REPLY"
    }
    response = json.loads(await ledger.submit_request(pool_handle, json.dumps(request)))
    assert response["op"] == expected_response["op"]
    assert response["result"]["identifier"] == expected_response["result"]["identifier"]
    assert response["result"]["dest"] == expected_response["result"]["dest"]
    assert response["result"]["data"] == expected_response["result"]["data"]
    assert response["result"]["type"] == expected_response["result"]["type"]


@pytest.mark.asyncio
async def test_submit_request_works_for_invalid_pool_handle(pool_handle, identity_my):
    (my_did, _) = identity_my

    get_nym_request = await ledger.build_get_nym_request(my_did, my_did)
    invalid_pool_handle = pool_handle + 1

    with pytest.raises(error.PoolLedgerInvalidPoolHandle):
        await ledger.submit_request(invalid_pool_handle, get_nym_request)


@pytest.mark.asyncio
async def test_nym_requests_works(pool_handle, wallet_handle, identity_trustee1, identity_my):
    (trustee_did, _) = identity_trustee1
    (my_did, my_ver_key) = identity_my

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_ver_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    get_nym_request = await ledger.build_get_nym_request(my_did, my_did)
    get_nym_response = await ensure_previous_request_applied(pool_handle, get_nym_request,
                                                             lambda response: response['result']['data'] is not None)
    get_nym_response = json.loads(await ledger.parse_get_nym_response(get_nym_response))

    assert my_did == get_nym_response['did']
    assert my_ver_key == get_nym_response['verkey']


@pytest.mark.asyncio
async def test_attrib_requests_works_for_raw_value(pool_handle, wallet_handle, identity_trustee1, identity_my):
    (trustee_did, _) = identity_trustee1
    (my_did, my_ver_key) = identity_my

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_ver_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    raw = "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"
    attrib_request = await ledger.build_attrib_request(my_did, my_did, None, raw, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, attrib_request)

    get_attrib_request = await ledger.build_get_attrib_request(my_did, my_did, "endpoint", None, None)
    get_attrib_response = json.loads(await ensure_previous_request_applied(pool_handle, get_attrib_request,
                                                                           lambda response: response['result'][
                                                                                                'data'] is not None))
    assert get_attrib_response['result']['data'] == raw


@pytest.mark.asyncio
async def test_schema_requests_works(pool_handle, wallet_handle, identity_my):
    (my_did, my_ver_key) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema_request = await ledger.build_schema_request(my_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)

    get_schema_request = await ledger.build_get_schema_request(my_did, schema_id)
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    await ledger.parse_get_schema_response(get_schema_response)


@pytest.mark.asyncio
async def test_get_txn_request_works(pool_handle, wallet_handle, identity_my):
    (my_did, _) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema = json.loads(schema_json)
    schema_request = await ledger.build_schema_request(my_did, schema_json)
    schema_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)
    seq_no = json.loads(schema_response)["result"]['txnMetadata']["seqNo"]

    get_txn_request = await ledger.build_get_txn_request(my_did, None, seq_no)
    get_txn_response = json.loads(
        await ensure_previous_request_applied(pool_handle, get_txn_request,
                                              lambda response: response['result']['data']['txnMetadata'][
                                                                   'seqNo'] is not None))

    received_schema = get_txn_response['result']['data']['txn']['data']['data']
    assert schema['name'] == received_schema['name']
    assert schema['version'] == received_schema['version']


@pytest.mark.asyncio
async def test_pool_config_request_works(pool_handle, wallet_handle, identity_trustee1):
    (did_trustee, _) = identity_trustee1

    request = await ledger.build_pool_config_request(did_trustee, False, False)
    response = json.loads(await ledger.sign_and_submit_request(pool_handle, wallet_handle, did_trustee, request))
    assert not response['result']['txn']['data']['writes']

    request = await ledger.build_pool_config_request(did_trustee, True, False)
    response = json.loads(await ledger.sign_and_submit_request(pool_handle, wallet_handle, did_trustee, request))
    assert response['result']['txn']['data']['writes']


@pytest.mark.asyncio
async def test_pool_upgrade_requests_works(pool_handle, wallet_handle, identity_trustee1):
    (did_trustee, _) = identity_trustee1
    next_year = datetime.datetime.now().year + 1

    schedule = {
        "Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv": str(next_year) + "-01-25T12:49:05.258870+00:00",
        "8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb": str(next_year) + "-01-25T13:49:05.258870+00:00",
        "DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya": str(next_year) + "-01-25T14:49:05.258870+00:00",
        "4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA": str(next_year) + "-01-25T15:49:05.258870+00:00"
    }

    request = await ledger.build_pool_upgrade_request(did_trustee, 'upgrade-python', '2.0.0', 'start',
                                                      'f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398',
                                                      None, json.dumps(schedule), None, False, False, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, did_trustee, request)

    request = await ledger.build_pool_upgrade_request(did_trustee, 'upgrade-python', '2.0.0', 'cancel',
                                                      'ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398',
                                                      None, None, None, False, False, None)
    json.loads(await ledger.sign_and_submit_request(pool_handle, wallet_handle, did_trustee, request))


async def ensure_previous_request_applied(pool_handle, checker_request, checker):
    for _ in range(3):
        response = json.loads(await ledger.submit_request(pool_handle, checker_request))
        try:
            if checker(response):
                return json.dumps(response)
        except TypeError as e:
            logger = logging.getLogger(__name__)
            logger.warning(e)
            logger.warning(response)
        time.sleep(5)
