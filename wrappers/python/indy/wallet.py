from .libindy import do_call, create_cb

from typing import Optional
from ctypes import *

import logging


async def create_wallet(pool_name: str,
                        name: str,
                        xtype: Optional[str],
                        config: Optional[str],
                        credentials: Optional[str]):
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
                  create_wallet.cb,
                  c_pool_name,
                  c_name,
                  c_xtype,
                  c_config,
                  c_credentials)

    logger.debug("create_wallet: <<<")
