import pytest
from vcx.error import ErrorCode, VcxError
from vcx.api.schema import Schema
import json

source_id = '123'
name = 'schema name'
version = '1.1.1'
attrs = ['attr1', 'attr2', 'height', 'weight']


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_schema():
    schema = await Schema.create(source_id, name, version, attrs, 0)
    assert schema.source_id == source_id
    assert schema.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_sets_schema_attrs():
    schema = await Schema.create(source_id, name, version, attrs, 0)
    assert schema.attrs == attrs


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    schema = await Schema.create(source_id, name, version, attrs, 0)
    data = await schema.serialize()
    assert data.get('data').get('source_id') == source_id
    assert data.get('data').get('name') == name
    assert schema.attrs == attrs


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        schema = Schema(source_id, name, version, attrs)
        schema.handle = 0
        await schema.serialize()
    assert ErrorCode.InvalidSchemaHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_and_payment_txn():
    schema = await Schema.create(source_id, name, version, attrs, 0)
    data = await schema.serialize()
    schema2 = await Schema.deserialize(data)
    assert schema2.source_id == data.get('data').get('source_id')
    txn = await schema.get_payment_txn()
    assert txn


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await Schema.deserialize(data)
    assert ErrorCode.InvalidSchema == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    schema = await Schema.create(source_id, name, version, attrs, 0)
    data1 = await schema.serialize()
    schema2 = await Schema.deserialize(data1)
    assert schema.source_id == schema2.source_id


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_release():
    with pytest.raises(VcxError) as e:
        schema = await Schema.create(source_id, name, version, attrs, 0)
        assert schema.handle > 0
        schema.release()
        await schema.serialize()
    assert ErrorCode.InvalidSchemaHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_lookup():
    schema_id = 'id1'
    schema = await Schema.lookup(source_id, schema_id)
    assert schema.attrs.sort() == ['sex', 'age', 'name', 'height'].sort()
    assert schema.name == 'test-licence'
    assert schema.handle > 0


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_get_schema_id_and_attributes():
    data = {
        "data": {
            "handle": 1553363118,
            "schema_id": "id1",
            "source_id": "Planet Express",
            "data": ["name", "account"],
            "name": "Account Ledger",
            "version": "1.1.1",
            "sequence_num": 481
        },
        "version": "1.0"
    }
    schema = await Schema.deserialize(data)
    assert isinstance(schema, Schema)
    seq_number = await schema.get_schema_id()
    assert seq_number == 'id1'
    assert schema.attrs == data['data']['data']


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_schema_prepare_for_endorser():
    schema = await Schema.prepare_for_endorser(source_id, name, version, attrs, 'V4SGRU86Z58d6TV7PBUe6f')
    assert schema.source_id == source_id
    assert schema.handle > 0
    assert schema.transaction


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_schema_update_state():
    schema = await Schema.prepare_for_endorser(source_id, name, version, attrs, 'V4SGRU86Z58d6TV7PBUe6f')
    assert 0 == await schema.get_state()
    assert 1 == await schema.update_state()
    assert 1 == await schema.get_state()


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_schema_get_state():
    schema = await Schema.create(source_id, name, version, attrs, 0)
    assert 1 == await schema.get_state()
