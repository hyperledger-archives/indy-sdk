import pytest
from vcx.error import ErrorCode, VcxError
from vcx.state import State
from vcx.api.issuer_credential import IssuerCredential
from vcx.api.connection import Connection

source_id = '1'
schema_no = 1234
cred_def_id = 'cred_def_id1'
attrs = {'key': 'value', 'key2': 'value2', 'key3': 'value3'}
name = 'Credential Name'
issuer_did = '8XFh8yBzrpJQmNyZzgoTqB'
phone_number = '8019119191'
price = '1'
req = {'libindy_cred_req': '', 'libindy_cred_req_meta': '', 'cred_def_id': '', 'tid': '', 'to_did': '', 'from_did': '',
       'version': '', 'mid': '', 'msg_ref_id': '123'}

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_issuer_credential():
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    assert issuer_credential.source_id == source_id
    assert issuer_credential.handle > 0
    assert await issuer_credential.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    data = await issuer_credential.serialize()
    assert data.get('data').get('source_id') == source_id
    assert data.get('data').get('credential_name') == name


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        issuer_credential = IssuerCredential(source_id, attrs, cred_def_id, name, price)
        issuer_credential.handle = 0
        await issuer_credential.serialize()
    assert ErrorCode.InvalidIssuerCredentialHandle == e.value.error_code
    assert 'Invalid Credential Issuer Handle' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    data = await issuer_credential.serialize()
    data['data']['handle'] = 99999
    data['data']['state'] = State.Expired
    issuer_credential2 = await IssuerCredential.deserialize(data)
    assert issuer_credential2.source_id == data.get('data').get('source_id')


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'data': { 'invalid': -99 } }
        await IssuerCredential.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code
    assert 'Invalid JSON string' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    data1 = await issuer_credential.serialize()
    print("data1: %s" %  data1)
    issuer_credential2 = await IssuerCredential.deserialize(data1)
    data2 = await issuer_credential2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    assert await issuer_credential.update_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        issuer_credential = IssuerCredential(source_id, attrs, cred_def_id, name, price)
        issuer_credential.handle = 0
        await issuer_credential.update_state()
    assert ErrorCode.InvalidIssuerCredentialHandle == e.value.error_code
    assert 'Invalid Credential Issuer Handle' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    assert await issuer_credential.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_issuer_credential_release():
    with pytest.raises(VcxError) as e:
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        assert issuer_credential.handle > 0
        issuer_credential.release()
        await issuer_credential.serialize()
    assert ErrorCode.InvalidIssuerCredentialHandle == e.value.error_code
    assert 'Invalid Credential Issuer Handle' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_offer():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    await issuer_credential.send_offer(connection)
    assert await issuer_credential.update_state() == State.OfferSent
    txn = await issuer_credential.get_payment_txn()
    assert(txn)


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_offer_with_invalid_state():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        data = await issuer_credential.serialize()
        data['data']['state'] = State.Expired
        issuer_credential2 = await IssuerCredential.deserialize(data)
        await issuer_credential2.send_offer(connection)
    assert ErrorCode.NotReady == e.value.error_code
    assert 'Object not ready for specified action' == e.value.error_msg

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_offer_with_bad_connection():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        await issuer_credential.send_offer(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code
    assert 'Invalid Connection Handle' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_credential():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
    await issuer_credential.send_offer(connection)
    assert await issuer_credential.update_state() == State.OfferSent
    # simulate consumer sending credential_req
    data = await issuer_credential.serialize()
    data['data']['state'] = State.RequestReceived
    data['data']['credential_request'] = req
    issuer_credential2 = await issuer_credential.deserialize(data)
    await issuer_credential2.send_credential(connection)
    assert await issuer_credential2.get_state() == State.Accepted


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_credential_with_invalid_issuer_credential():
    with pytest.raises(VcxError) as e:
        issuer_credential = IssuerCredential(source_id, attrs, cred_def_id, name, price)
        await issuer_credential.send_credential(Connection(source_id))
    assert ErrorCode.InvalidIssuerCredentialHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_credential_with_invalid_connection():
    with pytest.raises(VcxError) as e:
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        await issuer_credential.send_credential(Connection(source_id))
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_credential_with_no_prior_offer():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        issuer_credential = await IssuerCredential.create(source_id, attrs, cred_def_id, name, price)
        await issuer_credential.send_credential(connection)
    assert ErrorCode.NotReady == e.value.error_code

