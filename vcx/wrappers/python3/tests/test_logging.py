import pytest
from vcx.error import VcxError, error_message
from vcx.api.logging import VcxLogger
from ctypes import CFUNCTYPE, c_void_p, c_char_p, c_uint32, cast, c_int


def test_default_logging():
    try:
        VcxLogger.init_default()
    except VcxError as e:
        pytest.fail("Error in VcxLogger.init_default: %s", e)

    # try:
    #     logger = VcxLogger.get_logger()
    # except VcxError as e:
    #     pytest.fail("Error Getting Logger: %s", e)

    error_message(1000)


# Tests that a custom logger can be set and
# is utilized when a method calls a logging call
# (for example the error_message api call)
def test_set_logger():

    num_entries = c_int(0)

    assert(num_entries.value == 0)

    def _log_fn(_context, _level, _target, _message, _module_path, _file, _line):
        num_entries.value += 1

    c_converter = CFUNCTYPE(None, c_void_p, c_uint32, c_char_p, c_char_p, c_char_p, c_char_p, c_uint32)
    c_func = c_converter(_log_fn)

    try:
        VcxLogger.set_logger(c_func)
    except VcxError as e:
        pytest.fail("Error in VcxLogger.set_logger: %s", e)

    error_message(1000)
    assert(num_entries.value > 0)
