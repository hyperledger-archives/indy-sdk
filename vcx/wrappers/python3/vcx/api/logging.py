from vcx.common import do_call, create_cb
from ctypes import c_char_p, cast, c_uint32,c_long, c_void_p, CFUNCTYPE, POINTER
from vcx.cdll import _cdll
from vcx.error import ErrorCode, VcxError


class VcxLogger:

    @staticmethod
    def init_default():
        pattern = "info"
        c_pattern = c_char_p(pattern.encode('utf-8'))
        name = 'vcx_set_default_logger'
        err = getattr(_cdll(), name)(c_pattern)
        if err != ErrorCode.Success:
            raise VcxError(ErrorCode(err))
        # await do_call('vcx_set_default_logger', *c_pattern)

    @staticmethod
    def set_logger(c_log_fn):
        name = 'vcx_set_logger'
        getattr(_cdll(), name)(None, None, c_log_fn, None)

    @staticmethod
    def get_logger(context, enabled_cb, log_cb, flush_cb):
        name = 'vcx_get_logger'
        err = getattr(_cdll(), name)(context, enabled_cb, log_cb, flush_cb)
        if err != ErrorCode.Success:
            raise VcxError(ErrorCode(err))

