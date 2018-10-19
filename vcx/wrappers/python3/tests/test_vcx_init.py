import pytest
from vcx.error import ErrorCode, VcxError
from vcx.common import error_message, update_institution_info
from vcx.api.connection import Connection


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_vcx_init():
    pass


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_error_message():
    assert error_message(ErrorCode.NotReady) == 'Object not ready for specified action'


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_shutdown_works(shutdown):
    update_institution_info('name1', 'http://www.evernym.com')
    with pytest.raises(VcxError) as e:
        connection = await Connection.create('123')
        assert connection.handle > 0
        shutdown(True)
        await connection.serialize()
    assert ErrorCode.InvalidConnectionHandle == e.value.error_code
