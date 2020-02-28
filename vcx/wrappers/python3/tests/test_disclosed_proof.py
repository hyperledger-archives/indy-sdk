import pytest
from vcx.error import ErrorCode, VcxError
from vcx.state import State
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.connection import Connection
import json

connection_options = '{"connection_type":"SMS","phone":"8019119191","use_public_did":true}'
source_id = '1'
msg_id = '1'
request = {
    "@topic": {
        "mid": 9,
        "tid": 1
    },
    "@type": {
        "name": "PROOF_REQUEST",
        "version":"1.0"
    },
    "msg_ref_id": "ymy5nth",
    "proof_request_data": {
        "name": "Account Certificate",
        "nonce": "838186471541979035208225",
        "requested_attributes": {
            "business_2": {
                "name": "business"
            },
            "email_1": {
                "name": "email"
            },
            "name_0": {
                "name": "name"
            }
        },
        "requested_predicates": {},
        "version": "0.1"
    }
  }

proof_json = {
    "source_id": "id",
    "my_did": None,
    "my_vk": None,
    "state": 3,
    "proof_request": {
        "@type": {
            "name": "PROOF_REQUEST",
            "version": "1.0"
        },
        "@topic": {
            "mid": 9,
            "tid": 1
        },
        "proof_request_data": {
            "nonce": "838186471541979035208225",
            "name": "Account Certificate",
            "version": "0.1",
            "requested_attributes": {
                "business_2": {
                    "name": "business"
                },
                "email_1": {
                    "name": "email"
                },
                "name_0": {
                    "name": "name"
                }
            },
            "requested_predicates": {}
        },
        "msg_ref_id":
        "ymy5nth"
    },
    "link_secret_alias": "main",
    "their_did": None,
    "their_vk": None,
    "agent_did": None,
    "agent_vk": None
  }

proof_with_version = {
    "version": "1.0",
    "data": proof_json
}

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_disclosed_proof():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    assert disclosed_proof.source_id == source_id
    assert disclosed_proof.handle > 0
    assert await disclosed_proof.get_state() == State.RequestReceived

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_disclosed_proof_with_msgid():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)

    disclosed_proof = await DisclosedProof.create_with_msgid(source_id, connection, msg_id)
    assert disclosed_proof.source_id == source_id
    assert disclosed_proof.handle > 0
    assert disclosed_proof.proof_request
    assert await disclosed_proof.get_state() == State.RequestReceived

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    data = await disclosed_proof.serialize()
    assert data.get('data').get('source_id') == source_id


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        disclosed_proof = DisclosedProof(source_id)
        disclosed_proof.handle = 0
        await disclosed_proof.serialize()
    assert ErrorCode.InvalidDisclosedProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    data = await disclosed_proof.serialize()
    data['data']['state'] = State.Expired
    disclosed_proof2 = await DisclosedProof.deserialize(data)
    assert disclosed_proof2.source_id == data.get('data').get('source_id')
    assert await disclosed_proof2.get_state() == State.Expired


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'version': '1.0', "data": {'invalid': -99}}
        await DisclosedProof.deserialize(data)
    assert ErrorCode.InvalidJson == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    data1 = await disclosed_proof.serialize()
    disclosed_proof2 = await DisclosedProof.deserialize(data1)
    data2 = await disclosed_proof2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    assert await disclosed_proof.update_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_state_with_invalid_handle():
    with pytest.raises(VcxError) as e:
        disclosed_proof = DisclosedProof(source_id)
        disclosed_proof.handle = 0
        await disclosed_proof.update_state()
    assert ErrorCode.InvalidDisclosedProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_state():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    assert await disclosed_proof.get_state() == State.RequestReceived


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_disclosed_proof_release():
    with pytest.raises(VcxError) as e:
        disclosed_proof = await DisclosedProof.create(source_id, request)
        assert disclosed_proof.handle > 0
        disclosed_proof.release()
        await disclosed_proof.serialize()
    assert ErrorCode.InvalidDisclosedProofHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_proof():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    disclosed_proof = await DisclosedProof.deserialize(proof_with_version)
    await disclosed_proof.send_proof(connection)
    assert await disclosed_proof.get_state() == State.Accepted


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_send_proof_msg():
    disclosed_proof = await DisclosedProof.deserialize(proof_with_version)
    msg = await disclosed_proof.get_send_proof_msg()
    assert msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_send_proof_with_bad_connection():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        disclosed_proof = await DisclosedProof.create(source_id, request)
        await disclosed_proof.send_proof(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_reject_proof():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    disclosed_proof = await DisclosedProof.deserialize(proof_with_version)
    await disclosed_proof.reject_proof(connection)
    assert await disclosed_proof.get_state() == State.Rejected


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_reject_proof_msg():
    disclosed_proof = await DisclosedProof.deserialize(proof_with_version)
    msg = await disclosed_proof.get_reject_proof_msg()
    assert msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_reject_proof_with_bad_connection():
    with pytest.raises(VcxError) as e:
        connection = Connection(source_id)
        disclosed_proof = await DisclosedProof.create(source_id, request)
        await disclosed_proof.reject_proof(connection)
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_requests():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    await DisclosedProof.get_requests(connection)


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_creds_for_req():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    creds = await disclosed_proof.get_creds()
    assert creds == {"attrs":{"height_1":[{"cred_info":{"referent":"92556f60-d290-4b58-9a43-05c25aac214e","attrs":{"name":"Bob","height":"4'11","sex":"male","age":"111"},"schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2471","rev_reg_id":None,"cred_rev_id":None},"interval":None}],"zip_2":[{"cred_info":{"referent":"2dea21e2-1404-4f85-966f-d03f403aac71","attrs":{"address2":"101 Wilson Lane","city":"SLC","state":"UT","zip":"87121","address1":"101 Tela Lane"},"schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:Home Address:5.5.5","cred_def_id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2479","rev_reg_id":None,"cred_rev_id":None},"interval":None}]},"predicates":{}}


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_generate_proof():
    disclosed_proof = await DisclosedProof.create(source_id, request)
    # An error would be thrown if generate_proof failed
    assert await disclosed_proof.generate_proof({}, {}) is None


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_decline_presentation_request():
    connection = await Connection.create(source_id)
    await connection.connect(connection_options)
    disclosed_proof = await DisclosedProof.create(source_id, request)
    await disclosed_proof.decline_presentation_request(connection, 'reason')
