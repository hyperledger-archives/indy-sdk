from ctypes import *

import asyncio
import itertools
import logging
import os
from .error import VcxError, ErrorCode

LIBRARY = "libvcx.so"
_futures = {}
_futures_counter = itertools.count()


def do_call(name: str, *args):
    logger = logging.getLogger(__name__)
    logger.debug("do_call: >>> name: %s, args: %s", name, args)

    event_loop = asyncio.get_event_loop()
    future = event_loop.create_future()
    command_handle = next(_futures_counter)

    _futures[command_handle] = (event_loop, future)

    err = getattr(_cdll(), name)(command_handle,
                                 *args)

    logger.debug("do_call: Function %s returned err: %i", name, err)

    if err != ErrorCode.Success:
        logger.warning("_do_call: Function %s returned error %i", name, err)
        future.set_exception(VcxError(ErrorCode(err), error_message(err)))

    logger.debug("do_call: <<< %s", future)
    return future


def release(name, handle):
    logger = logging.getLogger(__name__)

    err = getattr(_cdll(), name)(handle)

    logger.debug("release: Function %s returned err: %i", name, err)

    if err != ErrorCode.Success:
        logger.warning("release: Function %s returned error %i", name, err)
        raise VcxError(ErrorCode(err))


def error_message(error_code: int) -> str:
    logger = logging.getLogger(__name__)

    name = 'vcx_error_c_message'
    c_error_code = c_uint32(error_code)
    c_err_msg = getattr(_cdll(), name)(c_error_code)

    err_msg = cast(c_err_msg , c_char_p).value.decode()
    logger.debug("error_message: Function %s returned error_message: %s", name, err_msg)

    return err_msg


def create_cb(cb_type: CFUNCTYPE, transform_fn=None):

    def _cb(command_handle: int, err: int, *args):
        if transform_fn:
            args = transform_fn(*args)
        _cxs_callback(command_handle, err, *args)

    res = cb_type(_cb)

    return res


def _cxs_callback(command_handle: int, err: int, *args):
    (event_loop, future) = _futures[command_handle]
    event_loop.call_soon_threadsafe(_cxs_loop_callback, command_handle, err, *args)


def _cxs_loop_callback(command_handle: int, err, *args):

    (event_loop, future) = _futures.pop(command_handle)

    if future.cancelled():
        print("_indy_loop_callback: Future was cancelled earlier")
    else:
        if err != ErrorCode.Success:
            future.set_exception(VcxError(ErrorCode(err), error_message(err)))
        else:
            if len(args) == 0:
                res = None
            elif len(args) == 1:
                (res,) = args
            else:
                res = args

            future.set_result(res)


def _cdll() -> CDLL:
    if not hasattr(_cdll, "cdll"):
        _cdll.cdll = _load_cdll()

    return _cdll.cdll


def _load_cdll() -> CDLL:
    file_dir = '/usr'
    path = os.path.join(file_dir, "lib", LIBRARY)
    try:
        res = CDLL(path)
        return res
    except OSError as e:
        raise e

