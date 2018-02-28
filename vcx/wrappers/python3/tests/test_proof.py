import pytest
import json
import random
from vcx.error import ErrorCode, VcxError
from vcx.state import State, ProofState
from vcx.api.proof import Proof
from vcx.api.connection import Connection

source_id = '123'
name = 'proof name'
phone_number = '8019119191'
requested_attrs = [{'name': 'a', 'issuer_did': '8XFh8yBzrpJQmNyZzgoTqB', 'schema_seq_no': 1},
                   {'name': 'b'},
                   {'name': 'c', 'issuer_did': '77Fh8yBzrpJQmNyZzgoTqB'}]
proof_msg = '{"version":"0.1","to_did":"BnRXf8yDMUwGyZVDkSENeq","from_did":"GxtnGN6ypZYgEqcftSQFnC","proof_request_id":"cCanHnpFAD","proofs":{"claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b":{"proof":{"primary_proof":{"eq_proof":{"revealed_attrs":{"state":"96473275571522321025213415717206189191162"},"a_prime":"921....546","e":"158....756","v":"114....069","m":{"address2":"140....691","city":"209....294","address1":"111...738","zip":"149....066"},"m1":"777....518","m2":"515....229"},"ge_proofs":[]},"non_revoc_proof":null},"schema_seq_no":15,"issuer_did":"4fUDR9R7fjwELRvH9JT6HH"}},"aggregated_proof":{"c_hash":"25105671496406009212798488318112715144459298495509265715919744143493847046467","c_list":[[72,245,38,"....",46,195,18]]},"requested_proof":{"revealed_attrs":{"attr_key_id":["claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b","UT","96473275571522321025213415717206189191162"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{}}}'


@pytest.mark.asyncio
async def test_create_proof_has_libindy_error_with_no_init():
    with pytest.raises(VcxError) as e:
        await Proof.create(source_id, '', [])
        assert ErrorCode.UnknownLibindyError == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_proof():
    proof = await Proof.create(source_id, name, requested_attrs)
    assert proof.source_id == source_id
    assert proof.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    proof = await Proof.create(source_id, name, requested_attrs)
    data = await proof.serialize()
    assert data.get('handle') == proof.handle
    assert data.get('source_id') == source_id
    assert data.get('name') == name


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
    proof = await Proof.create(source_id, name, requested_attrs)
    data = await proof.serialize()
    data['handle'] = random.randint(0, 99999)
    data['state'] = State.OfferSent
    proof2 = await Proof.deserialize(data)
    assert proof2.handle == data.get('handle')
    assert await proof2.get_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await Proof.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    proof = await Proof.create(source_id, name, requested_attrs)
    data1 = await proof.serialize()
    proof2 = await Proof.deserialize(data1)
    data2 = await proof2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_proof_release():
    with pytest.raises(VcxError) as e:
        proof = await Proof.create(source_id, name, requested_attrs)
        assert proof.handle > 0
        proof.release()
        await proof.serialize()
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    proof = await Proof.create(source_id, name, requested_attrs)
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
    await connection.connect(phone_number)
    proof = await Proof.create(source_id, name, requested_attrs)
    await proof.request_proof(connection)
    assert await proof.get_state() == State.OfferSent


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    proof = await Proof.create(source_id, name, requested_attrs)
    assert await proof.get_state() == State.Initialized


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof_with_invalid_connection():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        proof = await Proof.create(source_id, name, requested_attrs)
        connection.release()
        await proof.request_proof(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_request_proof_with_released_proof():
    with pytest.raises(VcxError) as e:
        connection = await Connection.create(source_id)
        await connection.connect(phone_number)
        proof = await Proof.create(source_id, name, requested_attrs)
        proof.release()
        await proof.request_proof(connection)
    assert ErrorCode.InvalidProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_proof_with_invalid_proof():
    connection = await Connection.create(source_id)
    await connection.connect(phone_number)
    proof = await Proof.create(source_id, name, requested_attrs)
    data = await proof.serialize()
    data['proof'] = json.loads(proof_msg)
    data['state'] = State.Accepted
    data['proof_state'] = ProofState.Invalid
    data['handle'] = random.randint(0, 99999)
    proof2 = await Proof.deserialize(data)
    await proof2.update_state()
    proof_data = await proof2.get_proof(connection)
    assert proof2.proof_state == ProofState.Invalid
    attrs = [{"schema_seq_no": 15,
              "issuer_did": "4fUDR9R7fjwELRvH9JT6HH",
              "claim_uuid": "claim::f22cc7c8-924f-4541-aeff-29a9aed9c46b",
              "attr_info": {"name": "state", "value": "UT", "type": "revealed"}}]
    assert proof_data[0] == attrs[0]
