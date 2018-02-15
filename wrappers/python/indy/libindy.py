from .error import ErrorCode, IndyError

from ctypes import *

import asyncio
import sys
import itertools
import logging

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
        future.set_exception(IndyError(ErrorCode(err)))

    logger.debug("do_call: <<< %s", future)
    return future


def create_cb(cb_type: CFUNCTYPE, transform_fn=None):
    logger = logging.getLogger(__name__)
    logger.debug("create_cb: >>> cb_type: %s", cb_type)

    def _cb(command_handle: int, err: int, *args):
        if transform_fn:
            args = transform_fn(*args)
        _indy_callback(command_handle, err, *args)

    res = cb_type(_cb)

    logger.debug("create_cb: <<< res: %s", res)
    return res


def _indy_callback(command_handle: int, err: int, *args):
    logger = logging.getLogger(__name__)
    logger.debug("_indy_callback: >>> command_handle: %i, err %i, args: %s", command_handle, err, args)

    (event_loop, future) = _futures[command_handle]
    event_loop.call_soon_threadsafe(_indy_loop_callback, command_handle, err, *args)

    logger.debug("_indy_callback: <<<")


def _indy_loop_callback(command_handle: int, err, *args):
    logger = logging.getLogger(__name__)
    logger.debug("_indy_loop_callback: >>> command_handle: %i, err %i, args: %s", command_handle, err, args)

    (event_loop, future) = _futures.pop(command_handle)

    if future.cancelled():
        logger.debug("_indy_loop_callback: Future was cancelled earlier")
    else:
        if err != ErrorCode.Success:
            logger.warning("_indy_loop_callback: Function returned error %i", err)
            future.set_exception(IndyError(ErrorCode(err)))
        else:
            if len(args) == 0:
                res = None
            elif len(args) == 1:
                (res,) = args
            else:
                res = args

            logger.debug("_indy_loop_callback: Function returned %s", res)
            future.set_result(res)

    logger.debug("_indy_loop_callback <<<")


def _cdll() -> CDLL:
    if not hasattr(_cdll, "cdll"):
        _cdll.cdll = _load_cdll()

    return _cdll.cdll


def _load_cdll() -> CDLL:
    logger = logging.getLogger(__name__)
    logger.debug("_load_cdll: >>>")

    libindy_prefix_mapping = {"darwin": "lib", "linux": "lib", "linux2": "lib", "win32": ""}
    libindy_suffix_mapping = {"darwin": ".dylib", "linux": ".so", "linux2": ".so", "win32": ".dll"}

    os_name = sys.platform
    logger.debug("_load_cdll: Detected OS name: %s", os_name)

    try:
        libindy_prefix = libindy_prefix_mapping[os_name]
        libindy_suffix = libindy_suffix_mapping[os_name]
    except KeyError:
        logger.error("_load_cdll: OS isn't supported: %s", os_name)
        raise OSError("OS isn't supported: %s", os_name)

    libindy_name = "{0}indy{1}".format(libindy_prefix, libindy_suffix)
    logger.debug("_load_cdll: Resolved libindy name is: %s", libindy_name)

    try:
        res = CDLL(libindy_name)
        logger.debug("_load_cdll: <<< res: %s", res)
        return res
    except OSError as e:
        logger.error("_load_cdll: Can't load libindy: %s", e)
        raise e
