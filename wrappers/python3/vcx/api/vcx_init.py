from ctypes import *
import logging
from vcx.common import do_call, create_cb


async def vcx_init(config_path: str) -> None:
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_init, "cb"):
        logger.debug("vcx_init: Creating callback")
        vcx_init.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_config_path = c_char_p(config_path.encode('utf-8'))

    result = await do_call('vcx_init',
                           c_config_path,
                           vcx_init.cb)

    logger.debug("vcx_init completed")
    return result


async def vcx_init_with_config(config: str) -> None:
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_init_with_config, "cb"):
        logger.debug("vcx_init_with_config: Creating callback")
        vcx_init_with_config.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_config = c_char_p(config.encode('utf-8'))

    result = await do_call('vcx_init_with_config',
                           c_config,
                           vcx_init_with_config.cb)

    logger.debug("vcx_init_with_config completed")
    return result



