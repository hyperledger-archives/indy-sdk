from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def create_wallet(pool_name: str,
                        name: str,
                        xtype: Optional[str],
                        config: Optional[str],
                        credentials: Optional[str]) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("create_wallet: >>> pool_name: %s, name: %s, xtype: %s, config: %s, credentials: %s",
                 pool_name,
                 name,
                 xtype,
                 config,
                 credentials)

    if not hasattr(create_wallet, "cb"):
        logger.debug("create_wallet: Creating callback")
        create_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_pool_name = c_char_p(pool_name.encode('utf-8'))
    c_name = c_char_p(name.encode('utf-8'))
    c_xtype = c_char_p(xtype.encode('utf-8')) if xtype is not None else None
    c_config = c_char_p(config.encode('utf-8')) if config is not None else None
    c_credentials = c_char_p(credentials.encode('utf-8')) if credentials is not None else None

    await do_call('indy_create_wallet',
                  c_pool_name,
                  c_name,
                  c_xtype,
                  c_config,
                  c_credentials,
                  create_wallet.cb)

    logger.debug("create_wallet: <<<")


async def open_wallet(name: str,
                      runtime_config: Optional[str],
                      credentials: Optional[str]) -> int:
    logger = logging.getLogger(__name__)
    logger.debug("open_wallet: >>> name: %s, runtime_config: %s, credentials: %s",
                 name,
                 runtime_config,
                 credentials)

    if not hasattr(open_wallet, "cb"):
        logger.debug("open_wallet: Creating callback")
        open_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_name = c_char_p(name.encode('utf-8'))
    c_runtime_config = c_char_p(runtime_config.encode('utf-8')) if runtime_config is not None else None
    c_credentials = c_char_p(credentials.encode('utf-8')) if credentials is not None else None

    res = await do_call('indy_open_wallet',
                        c_name,
                        c_runtime_config,
                        c_credentials,
                        open_wallet.cb)

    logger.debug("open_wallet: <<< res: %s", res)
    return res


async def close_wallet(handle: int) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("close_wallet: >>> handle: %i", handle)

    if not hasattr(close_wallet, "cb"):
        logger.debug("close_wallet: Creating callback")
        close_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_handle = c_int32(handle)

    await do_call('indy_close_wallet',
                  c_handle,
                  close_wallet.cb)

    logger.debug("close_wallet: <<<")


# pub extern fn indy_delete_wallet(command_handle: i32,
#                                    name: *const c_char,
#                                    credentials: *const c_char,
#                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode)>) -> ErrorCode {
async def delete_wallet(name: str,
                        credentials: Optional[str]) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("delete_wallet: >>> name: %s, credentials: %s",
                 name,
                 credentials)

    if not hasattr(delete_wallet, "cb"):
        logger.debug("delete_wallet: Creating callback")
        delete_wallet.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_name = c_char_p(name.encode('utf-8'))
    c_credentials = c_char_p(credentials.encode('utf-8')) if credentials is not None else None

    await do_call('indy_delete_wallet',
                  c_name,
                  c_credentials,
                  delete_wallet.cb)

    logger.debug("delete_wallet: <<<")
