from ctypes import *
import logging
from vcx.common import do_call, create_cb


async def vcx_agent_provision(config: str) -> None:
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_agent_provision, "cb"):
        logger.debug("vcx_agent_provision: Creating callback")
        vcx_agent_provision.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

    c_config = c_char_p(config.encode('utf-8'))

    result = await do_call('vcx_agent_provision_async',
                           c_config,
                           vcx_agent_provision.cb)

    logger.debug("vcx_agent_provision completed")
    return result

async def vcx_agent_update_info(config: str) -> None:
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_agent_update_info, "cb"):
        logger.debug("vcx_agent_update_info: Creating callback")
        vcx_agent_update_info.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_config = c_char_p(config.encode('utf-8'))

    result = await do_call('vcx_agent_update_info',
                           c_config,
                           vcx_agent_update_info.cb)

    logger.debug("vcx_agent_update_info completed")
    return result
