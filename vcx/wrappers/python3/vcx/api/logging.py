from vcx.common import do_call, create_cb
from ctypes import c_char_p, cast, c_uint32, c_void_p, CFUNCTYPE
from vcx.cdll import _cdll

    # logger = logging.getLogger(__name__)
    # name = 'vcx_error_c_message'
    # c_error_code = c_uint32(error_code)
    # c_err_msg = getattr(_cdll(), name)(c_error_code)
    # err_msg = cast(c_err_msg , c_char_p).value.decode()
    # logger.debug("error_message: Function %s[%s] returned error_message: %s", name, error_code, err_msg)
    # return err_msg


class VcxLogger:

    @staticmethod
    def init_default():
        pattern = "info"
        c_pattern = c_char_p(pattern.encode('utf-8'))
        name = 'vcx_set_default_logger'
        err = getattr(_cdll(), name)(c_pattern)
        # await do_call('vcx_set_default_logger', *c_pattern)

    @staticmethod
    def get_logger():
        return None, None, VcxLogger._log_fn, None

    @staticmethod
    def set_logger(c_log_fn):
        name = 'vcx_set_logger'
        getattr(_cdll(), name)(None, None, c_log_fn, None)
