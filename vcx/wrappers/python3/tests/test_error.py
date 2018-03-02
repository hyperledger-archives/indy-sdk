from vcx.error import ErrorCode


def test_error():
    assert ErrorCode.InvalidJson == 1016
