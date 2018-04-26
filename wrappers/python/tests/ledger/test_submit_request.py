import datetime
import json
import logging
import time

import pytest

from indy import ledger, anoncreds, blob_storage
from indy.error import ErrorCode, IndyError


@pytest.mark.asyncio
async def test_submit_request_works(pool_handle):
    request = {
        "reqId": 1491566332010860,
        "identifier": "Th7MpTaRZVRYnPiabds81Y",
        "operation": {
            "type": "105",
            "dest": "Th7MpTaRZVRYnPiabds81Y"
        },
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
async def test_submit_request_works_for_invalid_pool_handle(pool_handle, identity_my1):
    (my_did, _) = identity_my1

    get_nym_request = await ledger.build_get_nym_request(my_did, my_did)
    invalid_pool_handle = pool_handle + 1

    with pytest.raises(IndyError) as e:
        await ledger.submit_request(invalid_pool_handle, get_nym_request)
    assert ErrorCode.PoolLedgerInvalidPoolHandle == e.value.error_code


@pytest.mark.asyncio
async def test_send_nym_request_works_without_signature(pool_handle, identity_my1):
    (my_did, _) = identity_my1

    nym_request = await ledger.build_nym_request(my_did, my_did, None, None, None)

    response = await ledger.submit_request(pool_handle, nym_request)
    assert json.loads(response)['op'] == 'REQNACK'


@pytest.mark.asyncio
async def test_send_get_nym_request_works(pool_handle, identity_trustee1):
    (my_did, _) = identity_trustee1

    get_nym_request = await ledger.build_get_nym_request(my_did, my_did)

    response = json.loads(await ledger.submit_request(pool_handle, get_nym_request))
    assert response['result']['data'] is not None


@pytest.mark.asyncio
async def test_nym_requests_works(pool_handle, wallet_handle, identity_trustee1, identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_ver_key) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_ver_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    get_nym_request = await ledger.build_get_nym_request(my_did, my_did)
    get_nym_response = await ensure_previous_request_applied(pool_handle, get_nym_request,
                                                             lambda response: response['result']['data'] is not None)

    assert get_nym_response


@pytest.mark.asyncio
async def test_send_attrib_request_works_without_signature(pool_handle, identity_my1):
    (my_did, _) = identity_my1

    attrib_request = await ledger.build_attrib_request(my_did, my_did, None,
                                                       "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}", None)
    response = await ledger.submit_request(pool_handle, attrib_request)
    assert json.loads(response)['op'] == 'REQNACK'


@pytest.mark.asyncio
async def test_attrib_requests_works_for_raw_value(pool_handle, wallet_handle, identity_trustee1, identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_ver_key) = identity_my1

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
async def test_attrib_requests_works_for_hash_value(pool_handle, wallet_handle, identity_trustee1, identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_ver_key) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_ver_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    xhash = "83d907821df1c87db829e96569a11f6fc2e7880acba5e43d07ab786959e13bd3"
    attrib_request = await ledger.build_attrib_request(my_did, my_did, xhash, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, attrib_request)

    get_attrib_request = await ledger.build_get_attrib_request(my_did, my_did, None, xhash, None)
    get_attrib_response = json.loads(
        await ensure_previous_request_applied(pool_handle, get_attrib_request,
                                              lambda response: response['result']['data'] is not None))
    assert get_attrib_response['result']['data'] == xhash


@pytest.mark.asyncio
async def test_attrib_requests_works_for_enc_value(pool_handle, wallet_handle, identity_trustee1, identity_my1):
    (trustee_did, _) = identity_trustee1
    (my_did, my_ver_key) = identity_my1

    nym_request = await ledger.build_nym_request(trustee_did, my_did, my_ver_key, None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did, nym_request)

    enc = "aa3f41f619aa7e5e6b6d0de555e05331787f9bf9aa672b94b57ab65b9b66c3ea960b18a98e3834b1fc6cebf49f463b81fd6e3181"
    attrib_request = await ledger.build_attrib_request(my_did, my_did, None, None, enc)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, attrib_request)

    get_attrib_request = await ledger.build_get_attrib_request(my_did, my_did, None, None, enc)
    get_attrib_response = json.loads(
        await ensure_previous_request_applied(pool_handle, get_attrib_request,
                                              lambda response: response['result']['data'] is not None))
    assert get_attrib_response['result']['data'] == enc


@pytest.mark.asyncio
async def test_send_schema_request_works_without_signature(pool_handle, identity_my1):
    (my_did, _) = identity_my1
    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))

    schema_request = await ledger.build_schema_request(my_did, schema_json)

    response = await ledger.submit_request(pool_handle, schema_request)
    assert json.loads(response)['op'] == 'REQNACK'


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
async def test_send_node_request_works_without_signature(pool_handle, identity_my1):
    (my_did, _) = identity_my1

    node_data = {
        "node_ip": "10.0.0.100",
        "node_port": 9710,
        "client_ip": "10.0.0.100",
        "client_port": 9709,
        "alias": "Node5",
        "services": ["VALIDATOR"],
        "blskey": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    }

    node_request = await ledger.build_node_request(my_did, my_did, json.dumps(node_data))

    response = await ledger.submit_request(pool_handle, node_request)
    assert json.loads(response)['op'] == 'REQNACK'


@pytest.mark.asyncio
async def test_cred_def_requests_works(pool_handle, wallet_handle, identity_my):
    (my_did, my_ver_key) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))

    schema_request = await ledger.build_schema_request(my_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)

    get_schema_request = await ledger.build_get_schema_request(my_did, schema_id)
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, my_did, schema_json, "TAG", "CL",
                                                               json.dumps({"support_revocation": False}))

    cred_def_request = await ledger.build_cred_def_request(my_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, cred_def_request)

    get_cred_def_request = await ledger.build_get_cred_def_request(my_did, cred_def_id)
    get_cred_def_response = \
        await ensure_previous_request_applied(pool_handle, get_cred_def_request,
                                              lambda response: response['result']['seqNo'] is not None)
    await ledger.parse_get_cred_def_response(get_cred_def_response)


@pytest.mark.asyncio
async def test_revoc_reg_def_requests_works(pool_handle, wallet_handle, identity_my, path_home):
    (my_did, my_ver_key) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema_request = await ledger.build_schema_request(my_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)

    get_schema_request = await ledger.build_get_schema_request(my_did, schema_id)
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, my_did, schema_json, "TAG", "CL",
                                                               json.dumps({"support_revocation": True}))

    cred_def_request = await ledger.build_cred_def_request(my_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, cred_def_request)

    tails_writer_config = json.dumps({'base_dir': str(path_home.joinpath("tails")), 'uri_pattern': ''})
    tails_writer = await blob_storage.open_writer('default', tails_writer_config)

    (rev_reg_def_id, rev_reg_def_json, _) = \
        await anoncreds.issuer_create_and_store_revoc_reg(wallet_handle, my_did, None, 'tag1', cred_def_id,
                                                          '{"max_cred_num": 5}', tails_writer)

    revoc_reg_request = await ledger.build_revoc_reg_def_request(my_did, rev_reg_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, revoc_reg_request)

    get_revoc_reg_def_request = await ledger.build_get_revoc_reg_def_request(my_did, rev_reg_def_id)
    get_revoc_reg_def_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_def_request,
                                              lambda response: response['result']['seqNo'] is not None)
    await ledger.parse_get_revoc_reg_def_response(get_revoc_reg_def_response)


@pytest.mark.asyncio
async def test_revoc_reg_requests_works(pool_handle, wallet_handle, identity_my, path_home):
    (my_did, my_ver_key) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema_request = await ledger.build_schema_request(my_did, schema_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)

    get_schema_request = await ledger.build_get_schema_request(my_did, schema_id)
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, my_did, schema_json, "TAG", "CL",
                                                               json.dumps({"support_revocation": True}))

    cred_def_request = await ledger.build_cred_def_request(my_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, cred_def_request)

    tails_writer_config = json.dumps({'base_dir': str(path_home.joinpath("tails")), 'uri_pattern': ''})
    tails_writer = await blob_storage.open_writer('default', tails_writer_config)

    (rev_reg_def_id, rev_reg_def_json, rev_reg_entry_json) = \
        await anoncreds.issuer_create_and_store_revoc_reg(wallet_handle, my_did, None, 'tag1', cred_def_id,
                                                          '{"max_cred_num": 5}', tails_writer)

    revoc_reg_request = await ledger.build_revoc_reg_def_request(my_did, rev_reg_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, revoc_reg_request)

    revoc_reg_entry_request = \
        await ledger.build_revoc_reg_entry_request(my_did, rev_reg_def_id, "CL_ACCUM", rev_reg_entry_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, revoc_reg_entry_request)

    timestamp = int(time.time()) + 100
    get_revoc_reg_request = await ledger.build_get_revoc_reg_request(my_did, rev_reg_def_id, timestamp)
    get_revoc_reg_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_request,
                                              lambda response: response['result']['seqNo'] is not None)
    await ledger.parse_get_revoc_reg_response(get_revoc_reg_response)


@pytest.mark.asyncio
async def test_revoc_reg_delta_requests_works(pool_handle, wallet_handle, identity_my, path_home):
    (my_did, my_ver_key) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema_request = await ledger.build_schema_request(my_did, schema_json)
    schema_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)

    get_schema_request = await ledger.build_get_schema_request(my_did, schema_id)
    get_schema_response = \
        await ensure_previous_request_applied(pool_handle, get_schema_request,
                                              lambda response: response['result']['seqNo'] is not None)
    (schema_id, schema_json) = await ledger.parse_get_schema_response(get_schema_response)

    (cred_def_id, cred_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, my_did, schema_json, "TAG", "CL",
                                                               json.dumps({"support_revocation": True}))

    cred_def_request = await ledger.build_cred_def_request(my_did, cred_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, cred_def_request)

    tails_writer_config = json.dumps({'base_dir': str(path_home.joinpath("tails")), 'uri_pattern': ''})
    tails_writer = await blob_storage.open_writer('default', tails_writer_config)

    (rev_reg_def_id, rev_reg_def_json, rev_reg_entry_json) = \
        await anoncreds.issuer_create_and_store_revoc_reg(wallet_handle, my_did, None, 'tag1', cred_def_id,
                                                          '{"max_cred_num": 5}', tails_writer)

    revoc_reg_request = await ledger.build_revoc_reg_def_request(my_did, rev_reg_def_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, revoc_reg_request)

    revoc_reg_entry_request = \
        await ledger.build_revoc_reg_entry_request(my_did, rev_reg_def_id, "CL_ACCUM", rev_reg_entry_json)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, revoc_reg_entry_request)

    to = int(time.time()) + 100
    get_revoc_reg_delta_request = await ledger.build_get_revoc_reg_delta_request(my_did, rev_reg_def_id, None, to)
    get_revoc_reg_delta_response = \
        await ensure_previous_request_applied(pool_handle, get_revoc_reg_delta_request,
                                              lambda response: response['result']['seqNo'] is not None)
    await ledger.parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response)


@pytest.mark.asyncio
async def test_get_txn_request_works(pool_handle, wallet_handle, identity_my):
    (my_did, _) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema = json.loads(schema_json)
    schema_request = await ledger.build_schema_request(my_did, schema_json)
    schema_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)
    seq_no = json.loads(schema_response)["result"]["seqNo"]

    get_txn_request = await ledger.build_get_txn_request(my_did, seq_no)
    get_txn_response = json.loads(
        await ensure_previous_request_applied(pool_handle, get_txn_request,
                                              lambda response: response['result']['data']['seqNo'] is not None))

    received_schema = get_txn_response['result']['data']['data']
    assert schema['name'] == received_schema['name']
    assert schema['version'] == received_schema['version']


@pytest.mark.asyncio
async def test_get_txn_request_works_for_invalid_seq_no(pool_handle, wallet_handle, identity_my):
    (my_did, _) = identity_my

    (schema_id, schema_json) = \
        await anoncreds.issuer_create_schema(my_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))
    schema_request = await ledger.build_schema_request(my_did, schema_json)
    schema_response = await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did, schema_request)
    schema_id = json.loads(schema_response)["result"]["seqNo"]

    seq_no = schema_id + 1

    get_txn_request = await ledger.build_get_txn_request(my_did, seq_no)
    get_txn_response = json.loads(await ledger.submit_request(pool_handle, get_txn_request))
    assert not get_txn_response['result']['data']


@pytest.mark.asyncio
async def test_pool_config_request_works(pool_handle, wallet_handle, identity_trustee1):
    (did_trustee, _) = identity_trustee1

    request = await ledger.build_pool_config_request(did_trustee, False, False)
    response = json.loads(await ledger.sign_and_submit_request(pool_handle, wallet_handle, did_trustee, request))
    assert not response['result']['writes']

    request = await ledger.build_pool_config_request(did_trustee, True, False)
    response = json.loads(await ledger.sign_and_submit_request(pool_handle, wallet_handle, did_trustee, request))
    assert response['result']['writes']


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
                                                      None, json.dumps(schedule), None, False, False)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, did_trustee, request)

    request = await ledger.build_pool_upgrade_request(did_trustee, 'upgrade-python', '2.0.0', 'cancel',
                                                      'ac3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398',
                                                      None, None, None, False, False)
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
