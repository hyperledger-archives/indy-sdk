import pytest
import base64
import random
from vcx.error import ErrorCode, VcxError
from vcx.state import State
from vcx.api.connection import Connection


source_id = '123'
connection_options = '{"connection_type":"SMS","phone":"8019119191","use_public_did":true}'
details = '{"connReqId":"njjmmdg","senderAgencyDetail":{"DID":"YRuVCckY6vfZfX9kcQZe3u","endpoint":"52.38.32.107:80/agency/msg","verKey":"J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v"},"senderDetail":{"DID":"JZho9BzVAEk8jJ1hwrrDiZ","agentKeyDlgProof":{"agentDID":"JDF8UHPBTXigvtJWeeMJzx","agentDelegatedKey":"AP5SzUaHHhF5aLmyKHB3eTqUaREGKyVttwo5T4uwEkM4","signature":"JHSvITBMZiTEhpK61EDIWjQOLnJ8iGQ3FT1nfyxNNlxSngzp1eCRKnGC/RqEWgtot9M5rmTC8QkZTN05GGavBg=="},"logoUrl":"https://robohash.org/123","name":"Evernym","verKey":"AaEDsDychoytJyzk4SuzHMeQJGCtQhQHDitaic6gtiM1"},"statusCode":"MS-101","statusMsg":"message created","targetName":"there"}'


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_connection():
    connection = await Connection.create(source_id)
    assert connection.source_id == source_id
    assert connection.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_connection_connect():
    connection = await Connection.create(source_id)
    invite_details = await connection.connect(connection_options)
    assert invite_details
    await connection.delete()
    with pytest.raises(VcxError) as e:
        await connection.serialize()

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_connection_send_message():
    connection = await Connection.create(source_id)
    invite_details = await connection.connect(connection_options)
    assert invite_details
    with pytest.raises(VcxError) as e:
        msg_id = await connection.send_message("msg","type","title")
    assert ErrorCode.NotReady == e.value.error_code

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_connection_sign_data():
    connection = await Connection.create(source_id)
    invite_details = await connection.connect(connection_options)
    assert invite_details
    signature = await connection.sign_data(invite_details)
    assert signature

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_connection_verify_signature():
    connection = await Connection.create(source_id)
    invite_details = await connection.connect(connection_options)
    assert invite_details
    signature = await connection.verify_signature(invite_details, invite_details)
    assert signature

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_connection_with_invite_connect():
    connection = await Connection.create_with_details(source_id, details)
    invite = await connection.connect(connection_options)
    assert invite


@pytest.mark.asyncio
async def test_call_to_connect_with_bad_handle():
    with pytest.raises(VcxError) as e:
        invalid_connection = Connection(source_id)
        invalid_connection.handle = 0
        await invalid_connection.connect(connection_options)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_call_to_connect_state_not_initialized():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(connection_options)
        data = await connection.serialize()
        data['data']['state'] = 0
        data['data']['handle'] = random.randint(900, 99999)
        connection2 = await Connection.deserialize(data)
        await connection2.connect(connection_options)
    assert ErrorCode.ConnectionError == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    data = await connection.serialize()
    assert data.get('data').get('source_id') == source_id


@pytest.mark.asyncio
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        connection.handle = 0
        await connection.serialize()
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    data = await connection.serialize()
    connection2 = await Connection.deserialize(data)
    assert connection2.source_id == data.get('source_id')

    state = await connection2.get_state()
    assert state == State.OfferSent
    connection3 = connection


@pytest.mark.asyncio
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await Connection.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    data1 = await connection.serialize()
    connection2 = await Connection.deserialize(data1)
    data2 = await connection2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_connection_release():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        assert connection.handle > 0
        connection.release()
        await connection.serialize()
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    connection = await Connection.create(source_id)
    assert await connection.update_state() == State.Initialized
    await connection.connect(connection_options)
    assert await connection.update_state() == State.OfferSent


@pytest.mark.asyncio
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        connection.handle = 0
        await connection.update_state()
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    connection = await Connection.create(source_id)
    assert await connection.get_state() == State.Initialized
