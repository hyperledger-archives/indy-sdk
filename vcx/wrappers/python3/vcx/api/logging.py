from vcx.common import do_call, create_cb
from ctypes import c_char_p, cast, c_uint32, c_long, c_void_p, CFUNCTYPE, POINTER, Structure
from vcx.cdll import _cdll
from vcx.error import ErrorCode, VcxError
import ctypes
import sys


def default_logger():
    pattern = "info"
    c_pattern = c_char_p(pattern.encode('utf-8'))
    name = 'vcx_set_default_logger'
    err = getattr(_cdll(), name)(c_pattern)
    if err != ErrorCode.Success:
        raise VcxError(ErrorCode(err))


class Logger(ctypes.Structure):
    _fields_ = [
        ('a', c_uint32),
        ('log', CFUNCTYPE(None, c_uint32, c_char_p, c_char_p, c_char_p, c_char_p, c_uint32)),
        ('flush', CFUNCTYPE(None, c_void_p))
    ]

    def log_cb(self, _level, _target, _message, _module_path, _file, _line):
        self.log(_level, _target, _message, _module_path, _file, _line)


def set_logger_fn(context, _level, _target, _message, _module_path, _file, _line):
    _logger = cast(context, ctypes.POINTER(Logger))
    _logger[0].log_cb(_level, _target, _message, _module_path, _file, _line)


def flush_cb(void_pointer_context):
    _logger = cast(void_pointer_context, ctypes.POINTER(Logger))
    _logger[0].flush_cb()
    pass


def set_logger(user_set_logger_fn):
    logger = Logger()
    log_cb_converter = CFUNCTYPE(None, c_uint32, c_char_p, c_char_p, c_char_p, c_char_p, c_uint32)
    f2 = log_cb_converter(user_set_logger_fn)
    logger.log = f2

    c_flush_converter = CFUNCTYPE(None, c_void_p)
    c_flush_fn = c_flush_converter(flush_cb)
    logger.flush = c_flush_fn

    _logger_obj_ptr = ctypes.pointer(logger)
    void_pointer_context = ctypes.cast(_logger_obj_ptr, ctypes.c_void_p)

    set_logger_converter = CFUNCTYPE(None, c_void_p, c_uint32, c_char_p, c_char_p, c_char_p, c_char_p, c_uint32)
    c_set_logger_fn = set_logger_converter(set_logger_fn)

    name = 'vcx_set_logger'
    err = getattr(_cdll(), name)(void_pointer_context, None, c_set_logger_fn, c_flush_fn)
    if err != ErrorCode.Success:
        raise VcxError(ErrorCode(err))


def get_logger(context, enabled_cb, log_cb, flush_cb):
    name = 'vcx_get_logger'
    err = getattr(_cdll(), name)(context, enabled_cb, log_cb, flush_cb)
    if err != ErrorCode.Success:
        raise VcxError(ErrorCode(err))

# class VcxLogger(Structure):
#
#     @staticmethod
#     def init_default():
#         pattern = "info"
#         c_pattern = c_char_p(pattern.encode('utf-8'))
#         name = 'vcx_set_default_logger'
#         err = getattr(_cdll(), name)(c_pattern)
#         if err != ErrorCode.Success:
#             raise VcxError(ErrorCode(err))
#         # await do_call('vcx_set_default_logger', *c_pattern)
#
#
#
#     @staticmethod
#     def set_logger(user_set_logger_fn):
#         log_cb_converter = CFUNCTYPE(None, c_uint32, c_char_p, c_char_p, c_char_p, c_char_p, c_uint32)
#         f2 = log_cb_converter(user_set_logger_fn)
#
#         logger = Logger()
#         logger.b = f2
#         _logger_obj_ptr = ctypes.pointer(logger)
#         void_pointer_context = ctypes.cast(_logger_obj_ptr, ctypes.c_void_p)
#
#         set_logger_converter = CFUNCTYPE(None, c_void_p, c_uint32, c_char_p, c_char_p, c_char_p, c_char_p, c_uint32)
#         c_set_logger_fn = set_logger_converter(set_logger_fn)
#
#         def _flush_cb(void_pointer_context):
#             _logger = cast(void_pointer_context, ctypes.POINTER(Logger))
#             _logger[0].flush_cb()
#             pass
#
#         c_flush_converter = CFUNCTYPE(None, c_void_p)
#         c_flush_fn = c_flush_converter(_flush_cb)
#
#         name = 'vcx_set_logger'
#         err = getattr(_cdll(), name)(void_pointer_context, None, c_set_logger_fn, c_flush_fn)
#         if err != ErrorCode.Success:
#             raise VcxError(ErrorCode(err))

