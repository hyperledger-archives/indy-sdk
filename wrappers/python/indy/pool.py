from .libindy import do_call, create_cb

from ctypes import *

import logging

async def create_pool_ledger_config(config_name: str,
                                    config: str) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("create_pool_ledger_config: >>> config_name: %s, config: %s",
                 config_name,
                 config)

    if not hasattr(create_pool_ledger_config, "cb"):
        logger.debug("create_pool_ledger_config: Creating callback")
        create_pool_ledger_config.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config_name = c_char_p(config_name.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8'))

    res = await do_call('indy_create_pool_ledger_config',
                        create_pool_ledger_config.cb,
                        c_config_name,
                        c_config)

    logger.debug("create_pool_ledger_config: <<< res: %s", res)
    return res


async def open_pool_ledger(config_name: str,
                           config: str) -> int:
    logger = logging.getLogger(__name__)
    logger.debug("open_pool_ledger: >>> config_name: %s, config: %s",
                 config_name,
                 config)

    if not hasattr(open_pool_ledger, "cb"):
        logger.debug("open_pool_ledger: Creating callback")
        open_pool_ledger.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config_name = c_char_p(config_name.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8'))

    res = await do_call('indy_open_pool_ledger',
                        open_pool_ledger.cb,
                        c_config_name,
                        c_config)

    logger.debug("open_pool_ledger: <<< res: %s", res)
    return res


async def refresh_pool_ledger(handle: int) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("refresh_pool_ledger: >>> config_name: %s",
                 handle)

    if not hasattr(refresh_pool_ledger, "cb"):
        logger.debug("refresh_pool_ledger: Creating callback")
        refresh_pool_ledger.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_handle = c_int32(handle)

    res = await do_call('indy_refresh_pool_ledger',
                        refresh_pool_ledger.cb,
                        c_handle)

    logger.debug("refresh_pool_ledger: <<< res: %s", res)
    return res


async def close_pool_ledger(handle: int) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("close_pool_ledger: >>> config_name: %s",
                 handle)

    if not hasattr(close_pool_ledger, "cb"):
        logger.debug("close_pool_ledger: Creating callback")
        close_pool_ledger.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_handle = c_int32(handle)

    res = await do_call('indy_close_pool_ledger',
                        close_pool_ledger.cb,
                        c_handle)

    logger.debug("close_pool_ledger: <<< res: %s", res)
    return res


async def delete_pool_ledger_config(config_name: str) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("delete_pool_ledger_config: >>> config_name: %s",
                 config_name)

    if not hasattr(delete_pool_ledger_config, "cb"):
        logger.debug("delete_pool_ledger_config: Creating callback")
        delete_pool_ledger_config.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_config_name = c_char_p(config_name.encode('utf-8'))

    res = await do_call('indy_delete_pool_ledger_config',
                        delete_pool_ledger_config.cb,
                        c_config_name)

    logger.debug("delete_pool_ledger_config: <<< res: %s", res)
    return res
