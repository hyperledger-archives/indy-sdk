from .libindy import do_call, create_cb

from ctypes import *

import logging


async def create_reader_config(type_: str, config: str) -> int:

    logger = logging.getLogger(__name__)
    logger.debug("create_reader_config: >>> type_: %r, config: %r",
                 type_,
                 config)

    if not hasattr(create_reader_config, "cb"):
        logger.debug("create_reader_config: Creating callback")
        create_reader_config.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_type = c_char_p(type_.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8'))

    res = await do_call('indy_blob_storage_create_reader_config',
                        c_type,
                        c_config,
                        create_reader_config.cb)

    logger.debug("create_reader_config: <<< res: %r", res)
    return res
