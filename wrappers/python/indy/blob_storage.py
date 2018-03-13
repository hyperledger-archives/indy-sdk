from .libindy import do_call, create_cb

from ctypes import *

import logging


async def open_reader(type_: str,
                      config: str,
                      location: str,
                      hash: str) -> int:

    logger = logging.getLogger(__name__)
    logger.debug("open_reader: >>> type_: %r, config: %r, location: %r, hash: %r",
                 type_,
                 config,
                 location,
                 hash)

    if not hasattr(open_reader, "cb"):
        logger.debug("open_reader: Creating callback")
        open_reader.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    c_type = c_char_p(type_.encode('utf-8'))
    c_config = c_char_p(config.encode('utf-8'))
    c_location = c_char_p(location.encode('utf-8'))
    c_hash = c_char_p(hash.encode('utf-8'))

    res = await do_call('indy_blob_storage_open_reader',
                        c_type,
                        c_config,
                        c_location,
                        c_hash,
                        open_reader.cb)

    logger.debug("open_reader: <<< res: %r", res)
    return res
