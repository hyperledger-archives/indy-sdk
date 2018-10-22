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

