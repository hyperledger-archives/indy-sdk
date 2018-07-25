from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import json
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
            "timeout": int (optional), timeout for network request (in sec).
            "extended_timeout": int (optional), extended timeout for network request (in sec).
            "preordered_nodes": array<string> -  (optional), names of nodes which will have a priority during request sending:
                ["name_of_1st_prior_node",  "name_of_2nd_prior_node", .... ]
                Note: Not specified nodes will be placed in a random way.
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

async def list_pools() -> None:
    """
    Lists names of created pool ledgers
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("list_pools: >>> ")

    if not hasattr(list_pools, "cb"):
        logger.debug("list_pools: Creating callback")
        list_pools.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    res = await do_call('indy_list_pools',
                        list_pools.cb)
    res = json.loads(res.decode())
    logger.debug("list_pools: <<< res: %r", res)
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


async def set_protocol_version(protocol_version: int) -> None:
    """
    Set PROTOCOL_VERSION to specific version.

    There is a global property PROTOCOL_VERSION that used in every request to the pool and
    specified version of Indy Node which Libindy works.
    By default PROTOCOL_VERSION=1.

    :param protocol_version: Protocol version will be used:
        1 - for Indy Node 1.3
        2 - for Indy Node 1.4
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("set_protocol_version: >>> protocol_version: %r",
                 protocol_version)

    if not hasattr(delete_pool_ledger_config, "cb"):
        logger.debug("set_protocol_version: Creating callback")
        delete_pool_ledger_config.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    res = await do_call('indy_set_protocol_version',
                        protocol_version,
                        delete_pool_ledger_config.cb)

    logger.debug("set_protocol_version: <<< res: %r", res)
    return res