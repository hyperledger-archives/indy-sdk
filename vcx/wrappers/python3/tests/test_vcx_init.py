import pytest
from vcx.error import ErrorCode, VcxError
from vcx.common import error_message, shutdown
from vcx.api.connection import Connection


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_vcx_init():
    pass

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_vcx_init_with_config():
    pass

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_error_message():
    assert error_message(ErrorCode.NotReady) == 'Object not ready for specified action'

@pytest.mark.asyncio
async def test_shutdown_works(vcx_init_test_mode):
    with pytest.raises(VcxError) as e:
        connection = await Connection.create('123')
        assert connection.handle > 0
        shutdown(True)
        await connection.serialize()
    # If vcx_init_test_mode was called on a previous test, then
    # the error code will be InvalidURL.  Otherwise the error
    # will be InvalidConnectionHandle.
    assert (ErrorCode.InvalidConnectionHandle == e.value.error_code) or \
           (ErrorCode.InvalidUrl == e.value.error_code)

