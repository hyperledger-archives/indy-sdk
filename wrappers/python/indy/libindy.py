import json

from .error import ErrorCode, IndyError

from ctypes import *

import asyncio
import sys
import itertools
import logging
from logging import ERROR, WARNING, INFO, DEBUG, CRITICAL
from typing import Optional

TRACE = 5

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
        error = _get_indy_error(err)
        future.set_exception(error)

    logger.debug("do_call: <<< %s", future)
    return future


def do_call_sync(name: str, *args):
    logger = logging.getLogger(__name__)
    logger.debug("do_call_sync: >>> name: %s, args: %s", name, args)

    err = getattr(_cdll(), name)(*args)

    logger.debug("do_call_sync: <<< %s", err)
    return err


def create_cb(cb_type: CFUNCTYPE, transform_fn=None):
    logger = logging.getLogger(__name__)
    logger.debug("create_cb: >>> cb_type: %s", cb_type)

    def _cb(command_handle: int, err: int, *args):
        if transform_fn:
            args = transform_fn(*args)
        error = _get_indy_error(err)
        _indy_callback(command_handle, error, *args)

    res = cb_type(_cb)

    logger.debug("create_cb: <<< res: %s", res)
    return res


def _get_indy_error(err: int) -> IndyError:
    if err == ErrorCode.Success:
        return IndyError(ErrorCode(err))
    else:
        error_details = _get_error_details()
        error = IndyError(ErrorCode(err), error_details)
        return error


def _get_error_details() -> Optional[dict]:
    logger = logging.getLogger(__name__)
    logger.debug("_get_error_details: >>>")

    error_c = c_char_p()
    getattr(_cdll(), 'indy_get_current_error')(byref(error_c))
    error_details = json.loads(error_c.value.decode()) if error_c.value else None

    logger.debug("_get_error_details: <<< error_details: %s", error_details)
    return error_details


def _indy_callback(command_handle: int, err: IndyError, *args):
    logger = logging.getLogger(__name__)
    logger.debug("_indy_callback: >>> command_handle: %i, err %s, args: %s", command_handle, err, args)

    (event_loop, future) = _futures[command_handle]
    event_loop.call_soon_threadsafe(_indy_loop_callback, command_handle, err, *args)

    logger.debug("_indy_callback: <<<")


def _indy_loop_callback(command_handle: int, err, *args):
    logger = logging.getLogger(__name__)
    logger.debug("_indy_loop_callback: >>> command_handle: %i, err %s, args: %s", command_handle, err, args)

    (event_loop, future) = _futures.pop(command_handle)

    if future.cancelled():
        logger.debug("_indy_loop_callback: Future was cancelled earlier")
    else:
        if err.error_code != ErrorCode.Success:
            logger.warning("_indy_loop_callback: Function returned error %s", err)
            future.set_exception(err)
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
        _set_logger()

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


def _set_logger():
    logger = logging.getLogger(__name__)
    logging.addLevelName(TRACE, "TRACE")
    logging.basicConfig(level=CRITICAL)

    logger.debug("set_logger: >>>")

    def _log(context, level, target, message, module_path, file, line):
        libindy_logger = logger.getChild('native.' + target.decode().replace('::', '.'))

        level_mapping = {1: ERROR, 2: WARNING, 3: INFO, 4: DEBUG, 5: TRACE, }

        libindy_logger.log(level_mapping[level],
                           "\t%s:%d | %s",
                           file.decode(),
                           line,
                           message.decode())

    _set_logger.callbacks = {
        'enabled_cb': None,
        'log_cb': CFUNCTYPE(None, c_void_p, c_int, c_char_p, c_char_p, c_char_p, c_char_p, c_int)(_log),
        'flush_cb': None
    }

    do_call_sync('indy_set_logger',
                 None,
                 _set_logger.callbacks['enabled_cb'],
                 _set_logger.callbacks['log_cb'],
                 _set_logger.callbacks['flush_cb'])

    logger.debug("set_logger: <<<")


def set_runtime_config(config: str):
    """
     Set libindy runtime configuration. Can be optionally called to change current params.

     :param config: {
      "crypto_thread_pool_size": Optional<int> - size of thread pool for the most expensive crypto operations. (4 by default)
      "collect_backtrace": Optional<bool> - whether errors backtrace should be collected.
          Capturing of backtrace can affect library performance.
          NOTE: must be set before invocation of any other API functions.
      }
    """

    logger = logging.getLogger(__name__)
    logger.debug("set_runtime_config: >>> config: %r", config)

    c_config = c_char_p(config.encode('utf-8'))

    do_call_sync('indy_set_runtime_config',
                 c_config)

    logger.debug("set_runtime_config: <<<")
