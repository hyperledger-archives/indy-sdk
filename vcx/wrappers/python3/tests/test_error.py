import pytest

from vcx.api.utils import vcx_agent_provision
from vcx.error import ErrorCode, VcxError, error_message


def test_error():
    assert ErrorCode.InvalidJson == 1016


def test_c_error_msg():
    assert error_message(0) == 'Success'


def test_all_error_codes():
    max = 0
    assert(VcxError(1079).error_msg == "Wallet Not Found")
    for e in ErrorCode:
        assert(VcxError(int(e)) != "Unknown Error")
        max = int(e)

    assert(VcxError(max+1).error_msg == "Unknown Error")


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_error_details():
    with pytest.raises(VcxError) as e:
        await vcx_agent_provision("")
    assert ErrorCode.InvalidOption == e.value.error_code
    assert e.value.error_msg
    assert e.value.sdk_error_full_message
    assert e.value.sdk_error_cause
