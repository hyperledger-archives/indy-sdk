import pytest
from vcx.error import ErrorCode, VcxError
from vcx.state import State
from vcx.api.issuer_claim import IssuerClaim
from vcx.api.connection import Connection

source_id = '1'
schema_no = 1234
attrs = {'key': 'value', 'key2': 'value2', 'key3': 'value3'}
name = 'Claim Name'
issuer_did = '8XFh8yBzrpJQmNyZzgoTqB'
phone_number = '8019119191'


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_issuer_claim():
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    assert issuer_claim.source_id == source_id
    assert issuer_claim.handle > 0
    assert await issuer_claim.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    data = await issuer_claim.serialize()
    assert data.get('handle') == issuer_claim.handle
    assert data.get('source_id') == source_id
    assert data.get('claim_name') == name


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        issuer_claim = IssuerClaim(source_id, attrs, schema_no, name)
        issuer_claim.handle = 0
        await issuer_claim.serialize()
    assert ErrorCode.InvalidIssuerClaimHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    data = await issuer_claim.serialize()
    data['handle'] = 99999
    data['state'] = State.Expired
    issuer_claim2 = await IssuerClaim.deserialize(data)
    assert issuer_claim2.handle == data.get('handle')
    assert await issuer_claim2.get_state() == State.Expired


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await IssuerClaim.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    data1 = await issuer_claim.serialize()
    issuer_claim2 = await IssuerClaim.deserialize(data1)
    data2 = await issuer_claim2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    assert await issuer_claim.update_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        issuer_claim = IssuerClaim(source_id, attrs, schema_no, name)
        issuer_claim.handle = 0
        await issuer_claim.update_state()
    assert ErrorCode.InvalidIssuerClaimHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    assert await issuer_claim.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_issuer_claim_release():
    with pytest.raises(VcxError) as e:
        issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
        assert issuer_claim.handle > 0
        issuer_claim.release()
        await issuer_claim.serialize()
    assert ErrorCode.InvalidIssuerClaimHandle == e.value.error_code



@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_offer():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    await issuer_claim.send_offer(connection)
    assert await issuer_claim.update_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_offer_with_invalid_state():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
        data = await issuer_claim.serialize()
        data['handle'] = data['handle'] + 1
        data['state'] = State.Expired
        issuer_claim2 = await IssuerClaim.deserialize(data)
        await issuer_claim2.send_offer(connection)
    assert ErrorCode.NotReady == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_offer_with_bad_connection():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
        await issuer_claim.send_offer(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_claim():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
    await issuer_claim.send_offer(connection)
    assert await issuer_claim.update_state() == State.OfferSent
    # simulate consumer sending claim_req
    data = await issuer_claim.serialize()
    data['handle'] = data['handle'] + 1
    data['state'] = State.RequestReceived
    issuer_claim2 = await issuer_claim.deserialize(data)
    await issuer_claim2.send_claim(connection)
    assert await issuer_claim2.get_state() == State.Accepted


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_claim_with_invalid_issuer_claim():
    with pytest.raises(VcxError) as e:
        issuer_claim = IssuerClaim(source_id, attrs, schema_no, name)
        await issuer_claim.send_claim(Connection(source_id))
    assert ErrorCode.InvalidIssuerClaimHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_claim_with_invalid_connection():
    with pytest.raises(VcxError) as e:
        issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
        await issuer_claim.send_claim(Connection(source_id))
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_claim_with_no_prior_offer():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        issuer_claim = await IssuerClaim.create(source_id, attrs, schema_no, name)
        await issuer_claim.send_claim(connection)
    assert ErrorCode.NotReady == e.value.error_code

