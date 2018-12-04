from ctypes import c_char_p, c_uint32, c_void_p, CFUNCTYPE
from vcx.common import do_call_sync
from vcx.error import ErrorCode, VcxError


def default_logger():
    pattern = "info"
    c_pattern = c_char_p(pattern.encode('utf-8'))
    name = 'vcx_set_default_logger'
    err = do_call_sync(name, c_pattern)

    if err != ErrorCode.Success:
        raise VcxError(ErrorCode(err))


def set_logger(user_set_logger_fn, use_set_flush_fn=None):
    def _log(*args):
        user_set_logger_fn(*args[1:])

    def _flush(*args):
        if use_set_flush_fn:
            use_set_flush_fn(*args[1:])

    set_logger.callbacks = {
        'enabled_cb': None,
        'log_cb': CFUNCTYPE(None, c_void_p, c_uint32, c_char_p, c_char_p, c_char_p, c_char_p, c_uint32)(_log),
        'flush_cb': CFUNCTYPE(None, c_void_p)(_flush),
    }

    name = 'vcx_set_logger'
    err = do_call_sync(name, None, None, set_logger.callbacks['log_cb'], set_logger.callbacks['flush_cb'])

    if err != ErrorCode.Success:
        raise VcxError(ErrorCode(err))


def get_logger(context, enabled_cb, log_cb, flush_cb):
    name = 'vcx_get_logger'
    err = do_call_sync(name, context, enabled_cb, log_cb, flush_cb)

    if err != ErrorCode.Success:
        raise VcxError(ErrorCode(err))
