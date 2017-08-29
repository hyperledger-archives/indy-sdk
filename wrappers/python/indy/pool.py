from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def create_pool_ledger_config(config_name: str,
                                    config: Optional[str]) -> None:
    """
    Creates a new local pool ledger configuration that can be used later to connect pool nodes.

    :param config_name: Name of the pool ledger configuration.
    :param config: (optional) Pool configuration json. if NULL, then default config will be used. Example:
        {
            "genesis_txn": string (optional), A path to genesis transaction file. If NULL, then a default one will be used.
                           If file doesn't exists default one will be created.
        }
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_pool_ledger_config: >>> config_name: %r, config: %r",
                 config_name,
                 config)

    if not hasattr(create_pool_ledger_config, "cb"):
        logger.debug("create_pool_ledger_config: Creating callback")
        create_pool_ledger_config.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config_name = c_char_p(config_name.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8')) if config is not None else None

    res = await do_call('indy_create_pool_ledger_config',
                        c_config_name,
                        c_config,
                        create_pool_ledger_config.cb)

    logger.debug("create_pool_ledger_config: <<< res: %r", res)
    return res


async def open_pool_ledger(config_name: str,
                           config: Optional[str]) -> int:
    """
    Opens pool ledger and performs connecting to pool nodes.

    Pool ledger configuration with corresponded name must be previously created
    with indy_create_pool_ledger_config method.
    It is impossible to open pool with the same name more than once.

    :param config_name: Name of the pool ledger configuration.
    :param config: (optional) Runtime pool configuration json.
     if NULL, then default config will be used. Example:
        {
            "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
                             Defaults to true.
            "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
                               Use 0 to disable automatic refresh. Defaults to 24*60.
            "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
                              Defaults to 20000.
        }
    :return: Handle to opened pool to use in methods that require pool connection.
    """

    logger = logging.getLogger(__name__)
    logger.debug("open_pool_ledger: >>> config_name: %r, config: %r",
                 config_name,
                 config)

    if not hasattr(open_pool_ledger, "cb"):
        logger.debug("open_pool_ledger: Creating callback")
        open_pool_ledger.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_config_name = c_char_p(config_name.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8')) if config is not None else None

    res = await do_call('indy_open_pool_ledger',
                        c_config_name,
                        c_config,
                        open_pool_ledger.cb)

    logger.debug("open_pool_ledger: <<< res: %r", res)
    return res


async def refresh_pool_ledger(handle: int) -> None:
    """
    Refreshes a local copy of a pool ledger and updates pool nodes connections.

    :param handle: pool handle returned by indy_open_pool_ledger
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("refresh_pool_ledger: >>> config_name: %r",
                 handle)

    if not hasattr(refresh_pool_ledger, "cb"):
        logger.debug("refresh_pool_ledger: Creating callback")
        refresh_pool_ledger.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_handle = c_int32(handle)

    res = await do_call('indy_refresh_pool_ledger',
                        c_handle,
                        refresh_pool_ledger.cb)

    logger.debug("refresh_pool_ledger: <<< res: %r", res)
    return res


async def close_pool_ledger(handle: int) -> None:
    """
    Closes opened pool ledger, opened nodes connections and frees allocated resources.

    :param handle: pool handle returned by indy_open_pool_ledger.
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("close_pool_ledger: >>> config_name: %r",
                 handle)

    if not hasattr(close_pool_ledger, "cb"):
        logger.debug("close_pool_ledger: Creating callback")
        close_pool_ledger.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_handle = c_int32(handle)

    res = await do_call('indy_close_pool_ledger',
                        c_handle,
                        close_pool_ledger.cb)

    logger.debug("close_pool_ledger: <<< res: %r", res)
    return res


async def delete_pool_ledger_config(config_name: str) -> None:
    """
    Deletes created pool ledger configuration.

    :param config_name: Name of the pool ledger configuration to delete.
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("delete_pool_ledger_config: >>> config_name: %r",
                 config_name)

    if not hasattr(delete_pool_ledger_config, "cb"):
        logger.debug("delete_pool_ledger_config: Creating callback")
        delete_pool_ledger_config.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config_name = c_char_p(config_name.encode('utf-8'))

    res = await do_call('indy_delete_pool_ledger_config',
                        c_config_name,
                        delete_pool_ledger_config.cb)

    logger.debug("delete_pool_ledger_config: <<< res: %r", res)
    return res
