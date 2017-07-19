from tests.utils import pool, storage
from tests.utils.wallet import create_and_open_wallet
from indy import ledger, wallet, signus
from indy.pool import close_pool_ledger
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


@pytest.fixture
async def wallet_handle():
    handle = await create_and_open_wallet()
    yield handle
    await wallet.close_wallet(handle)


@pytest.fixture
async def pool_handle():
    handle = await pool.create_and_open_pool_ledger("pool_1")
    yield handle
    await close_pool_ledger(handle)


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
                    ",\"verkey\":\"~7TYfekw4GUagBnBVCqPjiC\"}",
            "type": "105",
        },
        "op": "REPLY"
    }
    response = json.loads((await ledger.submit_request(pool_handle, json.dumps(request))).decode())
    assert response == expected_response


@pytest.mark.asyncio
async def test_submit_request_works_for_invalid_pool_handle(pool_handle, wallet_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                          '{"seed":"000000000000000000000000Trustee1"}')

    get_nym_request = await ledger.build_get_nym_request(my_did.decode(), my_did.decode())
    invalid_pool_handle = pool_handle + 1
    try:
        await ledger.submit_request(invalid_pool_handle, get_nym_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.PoolLedgerInvalidPoolHandle)) == type(e) and \
               IndyError(ErrorCode.PoolLedgerInvalidPoolHandle).args == e.args


@pytest.mark.asyncio
async def test_send_nym_request_works_without_signature(pool_handle, wallet_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                          '{"seed":"00000000000000000000000000000My1"}')

    nym_request = await ledger.build_nym_request(my_did.decode(), my_did.decode(), None, None, None)
    try:
        await ledger.submit_request(pool_handle, nym_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.LedgerInvalidTransaction)) == type(e) and \
               IndyError(ErrorCode.LedgerInvalidTransaction).args == e.args


@pytest.mark.asyncio
async def test_send_get_nym_request_works(pool_handle, wallet_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                          '{"seed":"000000000000000000000000Trustee1"}')

    get_nym_request = await ledger.build_get_nym_request(my_did.decode(), my_did.decode())

    response = json.loads((await ledger.submit_request(pool_handle, get_nym_request.decode())).decode())
    assert response['result']['data'] is not None


@pytest.mark.asyncio
async def test_nym_requests_works(pool_handle, wallet_handle):
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    (my_did, my_ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                                   '{"seed":"00000000000000000000000000000My1"}')

    nym_request = await ledger.build_nym_request(trustee_did.decode(), my_did.decode(), my_ver_key.decode(), None, None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, trustee_did.decode(), nym_request.decode())
    get_nym_request = await ledger.build_get_nym_request(my_did.decode(), my_did.decode())
    response = json.loads((await ledger.submit_request(pool_handle, get_nym_request.decode())).decode())
    assert response['result']['data'] is not None


@pytest.mark.asyncio
async def test_send_attrib_request_works_without_signature(pool_handle, wallet_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                          '{"seed":"00000000000000000000000000000My1"}')

    attrib_request = await ledger.build_attrib_request(my_did.decode(), my_did.decode(), None,
                                                       "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}", None)
    try:
        await ledger.submit_request(pool_handle, attrib_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.LedgerInvalidTransaction)) == type(e) and \
               IndyError(ErrorCode.LedgerInvalidTransaction).args == e.args


@pytest.mark.asyncio
async def test_attrib_requests_works(pool_handle, wallet_handle):
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    (my_did, my_ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                                   '{"seed":"00000000000000000000000000000My1"}')

    attrib_request = await ledger.build_attrib_request(my_did.decode(), my_did.decode(), None,
                                                       "{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}", None)
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did.decode(), attrib_request.decode())
    get_attrib_request = await ledger.build_get_attrib_request(my_did.decode(), my_did.decode(), "endpoint")
    response = json.loads((await ledger.submit_request(pool_handle, get_attrib_request.decode())).decode())
    assert response['result']['data'] is not None


@pytest.mark.asyncio
async def test_send_schema_request_works_without_signature(pool_handle, wallet_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                          '{"seed":"00000000000000000000000000000My1"}')

    schema_data = {
        "name": "gvt2",
        "version": "2.0",
        "keys": ["name", "male"]
    }

    schema_request = await ledger.build_schema_request(my_did.decode(), json.dumps(schema_data))

    try:
        await ledger.submit_request(pool_handle, schema_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.LedgerInvalidTransaction)) == type(e) and \
               IndyError(ErrorCode.LedgerInvalidTransaction).args == e.args


@pytest.mark.asyncio
async def test_schema_requests_works(pool_handle, wallet_handle):
    (trustee_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                               '{"seed":"000000000000000000000000Trustee1"}')
    (my_did, my_ver_key, _) = await signus.create_and_store_my_did(wallet_handle,
                                                                   '{"seed":"00000000000000000000000000000My1"}')

    schema_data = {
        "name": "gvt2",
        "version": "2.0",
        "keys": ["name", "male"]
    }

    schema_request = await ledger.build_schema_request(my_did.decode(), json.dumps(schema_data))
    await ledger.sign_and_submit_request(pool_handle, wallet_handle, my_did.decode(), schema_request.decode())
    get_schema_data = {
        "name": "gvt2",
        "version": "2.0"
    }
    get_schema_request = await ledger.build_get_schema_request(my_did.decode(), my_did.decode(), json.dumps(get_schema_data))
    response = json.loads((await ledger.submit_request(pool_handle, get_schema_request.decode())).decode())
    assert response['result']['data'] is not None


@pytest.mark.asyncio
async def test_send_node_request_works_without_signature(pool_handle, wallet_handle):
    (my_did, _, _) = await signus.create_and_store_my_did(wallet_handle,
                                                          '{"seed":"00000000000000000000000000000My1"}')

    node_data = {
        "node_ip": "10.0.0.100",
        "node_port": 9710,
        "client_ip": "10.0.0.100",
        "client_port": 9709,
        "alias": "Node5",
        "services": ["VALIDATOR"]
    }

    node_request = await ledger.build_node_request(my_did.decode(), my_did.decode(), json.dumps(node_data))

    try:
        await ledger.submit_request(pool_handle, node_request.decode())
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.LedgerInvalidTransaction)) == type(e) and \
               IndyError(ErrorCode.LedgerInvalidTransaction).args == e.args
