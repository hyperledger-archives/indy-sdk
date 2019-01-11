from ctypes import *

import asyncio
import itertools
import logging
from .error import VcxError, ErrorCode
from vcx.cdll import _cdll

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
        future.set_exception(VcxError(ErrorCode(err)))

    logger.debug("do_call: <<< %s", future)
    return future


def do_call_sync(name: str, *args):
    logger = logging.getLogger(__name__)
    logger.debug("do_call_sync: >>> name: %s, args: %s", name, args)

    err = getattr(_cdll(), name)(*args)

    logger.debug("do_call_sync: <<< %s", err)
    return err


def release(name, handle):
    logger = logging.getLogger(__name__)

    err = do_call_sync(name, handle)

    logger.debug("release: Function %s returned err: %i", name, err)

    if err != ErrorCode.Success:
        logger.warning("release: Function %s returned error %i", name, err)
        raise VcxError(ErrorCode(err))


def get_version() -> str:
    logger = logging.getLogger(__name__)

    name = 'vcx_version'
    c_version = do_call_sync(name)

    version = cast(c_version , c_char_p).value.decode()
    logger.debug("error_message: Function %s returned version: %s", name, version)

    return version


def update_institution_info(institution_name: str, logo_url: str) -> None:
    logger = logging.getLogger(__name__)

    name = 'vcx_update_institution_info'
    c_name = c_char_p(institution_name.encode('utf-8'))
    c_logo_url = c_char_p(logo_url.encode('utf-8'))

    do_call_sync(name, c_name, c_logo_url)
    logger.debug("vcx_init_with_config completed")


def shutdown(delete_wallet: bool):
    c_delete = c_bool(delete_wallet)
    name = 'vcx_shutdown'
    err = do_call_sync(name, c_delete)

    if err != ErrorCode.Success:
        raise VcxError(ErrorCode(err))


def mint_tokens():
    name = 'vcx_mint_tokens'
    do_call_sync(name, None, None)


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
            future.set_exception(VcxError(ErrorCode(err)))
        else:
            if len(args) == 0:
                res = None
            elif len(args) == 1:
                (res,) = args
            else:
                res = args

            future.set_result(res)

