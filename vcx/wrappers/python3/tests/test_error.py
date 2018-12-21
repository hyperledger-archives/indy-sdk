import pytest
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

