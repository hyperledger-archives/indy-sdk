import pytest
from vcx.error import ErrorCode, VcxError
from vcx.state import State
from vcx.api.claim import Claim
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

claim_json = {
    'source_id': 'wrapper_tests',
    'state': 3,
    'claim_name': None,
    'claim_request': None,
    'claim_offer': {
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
async def test_create_claim():
    claim = await Claim.create(source_id, offer)
    assert claim.source_id == source_id
    assert claim.handle > 0
    assert await claim.get_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    claim = await Claim.create(source_id, offer)
    data = await claim.serialize()
    assert data.get('source_id') == source_id


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        claim = Claim(source_id)
        claim.handle = 0
        await claim.serialize()
    assert ErrorCode.InvalidClaimHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    claim = await Claim.create(source_id, offer)
    data = await claim.serialize()
    data['state'] = State.Expired
    claim2 = await Claim.deserialize(data)
    assert claim2.source_id == data.get('source_id')
    assert await claim2.get_state() == State.Expired


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await Claim.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    claim = await Claim.create(source_id, offer)
    data1 = await claim.serialize()
    claim2 = await Claim.deserialize(data1)
    data2 = await claim2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    claim = await Claim.create(source_id, offer)
    assert await claim.update_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        claim = Claim(source_id)
        claim.handle = 0
        await claim.update_state()
    assert ErrorCode.InvalidClaimHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    claim = await Claim.create(source_id, offer)
    assert await claim.get_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_claim_release():
    with pytest.raises(VcxError) as e:
        claim = await Claim.create(source_id, offer)
        assert claim.handle > 0
        claim.release()
        await claim.serialize()
    assert ErrorCode.InvalidClaimHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_request():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    claim = await Claim.deserialize(claim_json)
    await claim.send_request(connection)
    assert await claim.update_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_request_with_invalid_state():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        claim = await Claim.create(source_id, offer)
        await claim.send_request(connection)
    assert ErrorCode.CreateClaimFailed == e.value.error_code

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_request_with_bad_connection():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        claim = await Claim.create(source_id, offer)
        await claim.send_request(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_offers():
    connection = await Connection.create(source_id)
    await Claim.get_offers(connection)
