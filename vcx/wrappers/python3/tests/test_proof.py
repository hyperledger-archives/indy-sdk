import pytest
import json
import random
from vcx.error import ErrorCode, VcxError
from vcx.state import State, ProofState
from vcx.api.proof import Proof
from vcx.api.connection import Connection
from tests.conftest import proof_message

source_id = '123'
name = 'proof name'
connection_options = '{"connection_type":"SMS","phone":"8019119191","use_public_did":true}'
requested_attrs = [{"name": "age", "restrictions": [{"schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766" } ] }, { "name":"name", "restrictions": [ { "schema_id": "6XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"Faber Student Info", "schema_version":"1.0", "schema_issuer_did":"6XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"8XFh8yBzrpJQmNyZzgoTqB", "cred_def_id": "8XFh8yBzrpJQmNyZzgoTqB:3:CL:1766" }, { "schema_id": "5XFh8yBzrpJQmNyZzgoTqB:2:schema_name:0.0.11", "schema_name":"BYU Student Info", "schema_version":"1.0", "schema_issuer_did":"5XFh8yBzrpJQmNyZzgoTqB", "issuer_did":"66Fh8yBzrpJQmNyZzgoTqB", "cred_def_id": "66Fh8yBzrpJQmNyZzgoTqB:3:CL:1766"}]}]
revocation_interval = {}


@pytest.mark.asyncio
async def test_create_proof_has_libindy_error_with_no_init():
    with pytest.raises(VcxError) as e:
        await Proof.create(source_id, '', [], revocation_interval)
        assert ErrorCode.UnknownLibindyError == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_proof():
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    assert proof.source_id == source_id
    assert proof.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    data = await proof.serialize()
    assert data.get('data').get('source_id') == source_id
    assert data.get('data').get('name') == name


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        proof = Proof(source_id)
        proof.handle = 0
        await proof.serialize()
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    data = await proof.serialize()
    data['data']['state'] = State.OfferSent
    proof2 = await Proof.deserialize(data)
    assert proof2.source_id == data.get('data').get('source_id')
    assert await proof2.get_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'version': "1.0", 'data':{'invalid': -99}}
        await Proof.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code
    assert 'Invalid JSON string' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    data1 = await proof.serialize()
    proof2 = await Proof.deserialize(data1)
    data2 = await proof2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_proof_release():
    with pytest.raises(VcxError) as e:
        proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
        assert proof.handle > 0
        proof.release()
        await proof.serialize()
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    assert await proof.update_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        proof = Proof(source_id)
        proof.handle = 0
        await proof.update_state()
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    await proof.request_proof(connection)
    assert await proof.get_state() == State.OfferSent

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_message():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    await proof.request_proof(connection)
    assert await proof.get_state() == State.OfferSent
    state = await proof.update_state_with_message(proof_message)
    assert state == State.Accepted


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    assert await proof.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof_with_invalid_connection():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(connection_options)
        proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
        connection.release()
        await proof.request_proof(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof_with_released_proof():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(connection_options)
        proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
        proof.release()
        await proof.request_proof(connection)
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_proof_with_invalid_proof():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    proof = await Proof.create(source_id, name, requested_attrs, revocation_interval)
    data = await proof.serialize()
    data['data']['proof'] = {'version': '1.0', 'to_did': None, 'from_did': None, 'proof_request_id': None, "libindy_proof": "{\"proof_data\":123}"}
    data['data']['state'] = State.Accepted
    data['data']['proof_state'] = ProofState.Invalid
    proof2 = await Proof.deserialize(data)
    await proof2.update_state()
    proof_data = await proof2.get_proof(connection)
    print(proof_data)
    assert proof2.proof_state == ProofState.Invalid
    assert proof_data == {"proof_data": 123}
