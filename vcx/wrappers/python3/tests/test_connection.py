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
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_message():
    connection = await Connection.create(source_id)
    assert await connection.update_state() == State.Initialized
    await connection.connect(connection_options)
    message = '{ "statusCode": "MS-104", "payload": [ -126, -91, 64, 116, 121, 112, 101, -125, -92, 110, 97, 109, 101, -83, 67, 111, 110, 110, 82, 101, 113, 65, 110, 115, 119, 101, 114, -93, 118, 101, 114, -93, 49, 46, 48, -93, 102, 109, 116, -92, 106, 115, 111, 110, -92, 64, 109, 115, 103, -36, 1, 79, -48, -127, -48, -84, 115, 101, 110, 100, 101, 114, 68, 101, 116, 97, 105, 108, -48, -122, -48, -93, 68, 73, 68, -48, -74, 75, 119, 49, 57, 54, 87, 75, 69, 72, 77, 98, 85, 105, 86, 71, 99, 85, 76, 120, 56, 107, 82, -48, -80, 97, 103, 101, 110, 116, 75, 101, 121, 68, 108, 103, 80, 114, 111, 111, 102, -48, -125, -48, -88, 97, 103, 101, 110, 116, 68, 73, 68, -48, -74, 76, 115, 102, 102, 106, 72, 114, 69, 52, 86, 113, 75, 66, 114, 69, 69, 99, 99, 86, 89, 75, 86, -48, -79, 97, 103, 101, 110, 116, 68, 101, 108, 101, 103, 97, 116, 101, 100, 75, 101, 121, -48, -39, 44, 66, 113, 70, 52, 113, 119, 85, 97, 104, 68, 109, 114, 116, 115, 113, 54, 111, 88, 109, 77, 112, 116, 89, 105, 57, 76, 109, 109, 70, 121, 102, 57, 55, 76, 111, 103, 53, 75, 86, 69, 83, 98, 121, 105, -48, -87, 115, 105, 103, 110, 97, 116, 117, 114, 101, -48, -39, 88, 67, 76, 57, 90, 65, 113, 119, 72, 82, 54, 70, 110, 112, 106, 118, 49, 106, 80, 47, 115, 121, 103, 65, 43, 74, 78, 57, 74, 104, 120, 69, 65, 68, 86, 117, 101, 71, 88, 83, 101, 90, 54, 73, 72, 75, 97, 43, 52, 106, 57, 105, 108, 82, 111, 74, 49, 119, 76, 56, 66, 121, 54, 119, 97, 117, 86, 56, 113, 72, 86, 71, 49, 71, 74, 112, 101, 49, 71, 79, 106, 67, 105, 108, 101, 65, 65, 61, 61, -48, -89, 108, 111, 103, 111, 85, 114, 108, -48, -64, -48, -92, 110, 97, 109, 101, -48, -64, -48, -87, 112, 117, 98, 108, 105, 99, 68, 73, 68, -48, -64, -48, -90, 118, 101, 114, 75, 101, 121, -48, -39, 44, 66, 75, 84, 50, 67, 85, 78, 71, 66, 82, 107, 81, 67, 104, 54, 118, 85, 89, 118, 65, 111, 110, 101, 107, 110, 54, 88, 75, 122, 122, 122, 86, 68, 90, 107, 98, 114, 74, 85, 56, 86, 104, 99, 114 ], "senderDID": "NsQ1rvm6TrsHx1TB4xEh55", "uid": "owm5yta", "type\": "connReqAnswer", "deliveryDetails": [] }'
    assert await connection.update_state_with_message(message) == State.Accepted


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
