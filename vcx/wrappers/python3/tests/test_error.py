from vcx.error import ErrorCode
from vcx.common import error_message

def test_error():
    assert ErrorCode.InvalidJson == 1016


def test_c_error_msg():
    assert error_message(0) == 'Success'
