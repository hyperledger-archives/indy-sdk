import pytest
from vcx.error import ErrorCode, VcxError
from vcx.api.claim_def import ClaimDef

source_id = '123'
name = 'schema name'
schema_no = 44


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_create_claim_def():
    claim_def = await ClaimDef.create(source_id, name, schema_no, False)
    assert claim_def.source_id == source_id
    assert claim_def.handle > 0
    assert claim_def.schema_no == schema_no


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize():
    claim_def = await ClaimDef.create(source_id, name, schema_no, False)
    data = await claim_def.serialize()
    assert data['handle'] == claim_def.handle
    assert data['source_id'] == source_id
    assert data['name'] == name


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_with_bad_handle():
    with pytest.raises(VcxError) as e:
        claim_def = await ClaimDef.create(source_id, name, schema_no, False)
        claim_def.handle = 0
        await claim_def.serialize()
    assert ErrorCode.InvalidClaimDefHandle == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize():
    claim_def = await ClaimDef.create(source_id, name, schema_no, False)
    data = await claim_def.serialize()
    claim_def2 = await ClaimDef.deserialize(data)
    assert claim_def2.handle == data.get('handle')


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_deserialize_with_invalid_data():
    with pytest.raises(VcxError) as e:
        data = {'invalid': -99}
        await ClaimDef.deserialize(data)
    assert ErrorCode.InvalidClaimDef == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_serialize_deserialize_and_then_serialize():
    claim_def = await ClaimDef.create(source_id, name, schema_no, False)
    data1 = await claim_def.serialize()
    claim_def2 = await ClaimDef.deserialize(data1)
    data2 = await claim_def2.serialize()
    assert data1 == data2


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_release():
    with pytest.raises(VcxError) as e:
        claim_def = await ClaimDef.create(source_id, name, schema_no, False)
        assert claim_def.handle > 0
        claim_def.release()
        await claim_def.serialize()
    assert ErrorCode.InvalidClaimDefHandle == e.value.error_code

