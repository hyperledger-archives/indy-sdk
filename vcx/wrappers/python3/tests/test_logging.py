import pytest
from vcx.error import VcxError, error_message, ErrorCode
from vcx.api.logging import set_logger, default_logger, get_logger
from ctypes import CFUNCTYPE, c_void_p, c_char_p, c_uint32, cast, c_int, c_bool, POINTER
import ctypes


def test_default_logging():
    try:
        default_logger()
    except VcxError as e:
        pytest.fail("Error in VcxLogger.init_default: %s", e)

    with pytest.raises(VcxError) as e:
        default_logger()

    assert ErrorCode.LoggingError == e.value.error_code

    error_message(1000)


# Tests that a custom logger can be set and
# is utilized when a method calls a logging call
# (for example the error_message api call)
#
# This test is skipped because Logger Cannot be
# Initialized twice in the same process
@pytest.mark.skip
def test_set_logger():
    num_entries = c_int(0)

    assert (num_entries.value == 0)

    def _log_fn(_level, _target, _message, _module_path, _file, _line):
        num_entries.value += 1

    try:
        set_logger(_log_fn)
    except VcxError as e:
        pytest.fail("Error in VcxLogger.set_logger: %s", e)

    error_message(1000)
    assert (num_entries.value > 0)


# These address should align with the addresses produced by the
# logging of the api call in libvcx.
#
# This test is skipped because Logger Cannot be
# Initialized twice in the same process
@pytest.mark.skip
def test_get_logger():
    try:
        default_logger()
    except VcxError as e:
        pytest.fail("Error in VcxLogger.init_default: %s", e)
    error_message(1000)

    int_array256 = c_uint32 * 256
    context = int_array256(0)
    enabled_cb = int_array256(0)
    log_cb = int_array256(0)
    flush_cb = int_array256(0)
    get_logger(context, enabled_cb, log_cb, flush_cb)

    print('_context address: %x' % ctypes.addressof(context))
    print('_enabled_cb address: %x' % ctypes.addressof(enabled_cb))
    print('_log_cb address: %x' % ctypes.addressof(log_cb))
    print('_flush_cb address: %x' % ctypes.addressof(flush_cb))
