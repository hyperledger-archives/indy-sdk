import pytest
from vcx.error import ErrorCode, VcxError
from vcx.api.credential_def import CredentialDef

source_id = '123'
schema_id = 'schema_id1'
name = 'schema name'
schema_no = 44


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_credential_def():
    credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
    assert credential_def.source_id == source_id
    assert credential_def.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
    data = await credential_def.serialize()
    assert data['data']['source_id'] == source_id
    assert data['data']['name'] == name


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
        credential_def.handle = 0
        await credential_def.serialize()
    assert ErrorCode.InvalidCredentialDefHandle == e.value.error_code
    assert 'Invalid Credential Definition handle' == e.value.error_msg

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
    data = await credential_def.serialize()
    credential_def2 = await CredentialDef.deserialize(data)
    assert credential_def2.source_id == data.get('data').get('source_id')


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await CredentialDef.deserialize(data)
    assert ErrorCode.InvalidCredentialDef == e.value.error_code
    assert 'Credential Def not in valid json' == e.value.error_msg

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
    data1 = await credential_def.serialize()
    credential_def2 = await CredentialDef.deserialize(data1)
    data2 = await credential_def2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_release():
    with pytest.raises(VcxError) as e:
        credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
        assert credential_def.handle > 0
        credential_def.release()
        await credential_def.serialize()
    assert ErrorCode.InvalidCredentialDefHandle == e.value.error_code
    assert 'Invalid Credential Definition handle' == e.value.error_msg


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_cred_def_id_and_payment_txn():
    credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
    assert await credential_def.get_cred_def_id() == '2hoqvcwupRTUNkXn6ArYzs:3:CL:2471'
    txn = await credential_def.get_payment_txn()
    assert txn


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_credential_def_prepare_for_endorser():
    credential_def = await CredentialDef.prepare_for_endorser(source_id, name, schema_id, 'V4SGRU86Z58d6TV7PBUe6f')
    assert credential_def.source_id == source_id
    assert credential_def.handle > 0
    assert credential_def.transaction


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_schema_update_state():
    credential_def = await CredentialDef.prepare_for_endorser(source_id, name, schema_id, 'V4SGRU86Z58d6TV7PBUe6f')
    assert 0 == await credential_def.get_state()
    assert 1 == await credential_def.update_state()
    assert 1 == await credential_def.get_state()


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_schema_get_state():
    credential_def = await CredentialDef.create(source_id, name, schema_id, 0)
    assert 1 == await credential_def.get_state()

