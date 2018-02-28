import pytest
from vcx.error import ErrorCode
from vcx.common import error_message


@pytest.mark.asyncio
async def test_vcx_init(vcx_init_test_mode):
    pass


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_error_message():
    assert await error_message(ErrorCode.NotReady) == 'Object not ready for specified action'
