from .libindy import do_call, create_cb

from ctypes import *

import logging


async def open_reader(type_: str, config: str) -> int:

    logger = logging.getLogger(__name__)
    logger.debug("open_reader: >>> type_: %r, config: %r",
                 type_,
                 config)

    if not hasattr(open_reader, "cb"):
        logger.debug("open_reader: Creating callback")
        open_reader.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_type = c_char_p(type_.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8'))

    res = await do_call('indy_open_blob_storage_reader',
                        c_type,
                        c_config,
                        open_reader.cb)

    logger.debug("open_reader: <<< res: %r", res)
    return res

async def open_writer(type_: str, config: str) -> int:

    logger = logging.getLogger(__name__)
    logger.debug("open_writer: >>> type_: %r, config: %r",
                 type_,
                 config)

    if not hasattr(open_writer, "cb"):
        logger.debug("open_writer: Creating callback")
        open_writer.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_type = c_char_p(type_.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8'))

    res = await do_call('indy_open_blob_storage_writer',
                        c_type,
                        c_config,
                        open_writer.cb)

    logger.debug("open_writer: <<< res: %r", res)
    return res
