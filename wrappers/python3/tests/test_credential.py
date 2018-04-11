import pytest
from vcx.error import ErrorCode, VcxError
from vcx.state import State
from vcx.api.credential import Credential
from vcx.api.connection import Connection

phone_number = '8019119191'
source_id = '1'
offer = {
    'msg_type': 'CLAIM_OFFER',
    'version': '0.1',
    'to_did': 'LtMgSjtFcyPwenK9SHCyb8',
    'from_did': 'LtMgSjtFcyPwenK9SHCyb8',
    'claim': {
      'account_num': [
        '8BEaoLf8TBmK4BUyX8WWnA'
      ],
      'name_on_account': [
        'Alice'
      ]
    },
    'schema_seq_no': 48,
    'issuer_did': 'Pd4fnFtRBcMKRVC2go5w3j',
    'claim_name': 'Account Certificate',
    'claim_id': '3675417066',
    'msg_ref_id':  None
  }

credential_json = {
    'source_id': 'wrapper_tests',
    'state': 3,
    'credential_name': None,
    'credential_request': None,
    'credential_offer': {
        'msg_type': 'CLAIM_OFFER',
        'version': '0.1',
        'to_did': 'LtMgSjtFcyPwenK9SHCyb8',
        'from_did': 'LtMgSjtFcyPwenK9SHCyb8',
        'claim': {'account_num': ['8BEaoLf8TBmK4BUyX8WWnA'], 'name_on_account': ['Alice']},
        'schema_seq_no': 48,
        'issuer_did': 'Pd4fnFtRBcMKRVC2go5w3j',
        'claim_name': 'Account Certificate',
        'claim_id': '3675417066',
        'msg_ref_id': 'ymy5nth'
    },
    'link_secret_alias': 'main',
    'msg_uid': None,
    'agent_did': None,
    'agent_vk': None,
    'my_did': None,
    'my_vk': None,
    'their_did': None,
    'their_vk': None
  }


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_credential():
    credential = await Credential.create(source_id, offer)
    assert credential.source_id == source_id
    assert credential.handle > 0
    assert await credential.get_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    credential = await Credential.create(source_id, offer)
    data = await credential.serialize()
    assert data.get('source_id') == source_id


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        credential = Credential(source_id)
        credential.handle = 0
        await credential.serialize()
    assert ErrorCode.InvalidCredentialHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    credential = await Credential.create(source_id, offer)
    data = await credential.serialize()
    data['state'] = State.Expired
    credential2 = await Credential.deserialize(data)
    assert credential2.source_id == data.get('source_id')
    assert await credential2.get_state() == State.Expired


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await Credential.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    credential = await Credential.create(source_id, offer)
    data1 = await credential.serialize()
    credential2 = await Credential.deserialize(data1)
    data2 = await credential2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    credential = await Credential.create(source_id, offer)
    assert await credential.update_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        credential = Credential(source_id)
        credential.handle = 0
        await credential.update_state()
    assert ErrorCode.InvalidCredentialHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    credential = await Credential.create(source_id, offer)
    assert await credential.get_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_credential_release():
    with pytest.raises(VcxError) as e:
        credential = await Credential.create(source_id, offer)
        assert credential.handle > 0
        credential.release()
        await credential.serialize()
    assert ErrorCode.InvalidCredentialHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_request():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    credential = await Credential.deserialize(credential_json)
    await credential.send_request(connection)
    assert await credential.update_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_request_with_invalid_state():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        credential = await Credential.create(source_id, offer)
        await credential.send_request(connection)
    assert ErrorCode.CreateCredentialFailed == e.value.error_code

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_request_with_bad_connection():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        credential = await Credential.create(source_id, offer)
        await credential.send_request(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_offers():
    connection = await Connection.create(source_id)
    await Credential.get_offers(connection)
