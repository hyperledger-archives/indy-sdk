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
    assert data.get('source_id') == source_id
    assert data.get('name') == name
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
    assert schema2.source_id == data.get('source_id')
    with pytest.raises(VcxError) as e:
        await schema.get_payment_txn()
    assert ErrorCode.NotReady == e.value.error_code


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
        "handle": 1553363118,
        "schema_id": "id1",
        "source_id": "Planet Express",
        "data": ["name", "account"],
        "name": "Account Ledger",
        "version": "1.1.1",
        "sequence_num": 481
    }
    schema = await Schema.deserialize(data)
    assert isinstance(schema, Schema)
    seq_number = await schema.get_schema_id()
    assert seq_number == 'id1'
    assert schema.attrs == data['data']
