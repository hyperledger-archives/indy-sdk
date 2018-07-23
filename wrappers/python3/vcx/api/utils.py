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

async def vcx_ledger_get_fees() -> str:
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_ledger_get_fees, "cb"):
        logger.debug("vcx_ledger_get_fees: Creating callback")
        vcx_ledger_get_fees.cb = create_cb(CFUNCTYPE(None, c_uint32))

    result = await do_call('vcx_ledger_get_fees',
                           vcx_ledger_get_fees.cb)

    logger.debug("vcx_ledger_get_fees completed")
    return result

async def vcx_messages_download(status: str = None, uids: str = None, pw_dids: str = None) -> str:
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_messages_download, "cb"):
        logger.debug("vcx_messages_download: Creating callback")
        vcx_messages_download.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

    if status:
        c_status = c_char_p(status.encode('utf-8'))
    else:
        c_status = None

    if uids:
        c_uids = c_char_p(uids.encode('utf-8'))
    else:
        c_uids = None

    if pw_dids:
        c_pw_dids = c_char_p(pw_dids.encode('utf-8'))
    else:
        c_pw_dids = None

    result = await do_call('vcx_messages_download',
                           c_status,
                           c_uids,
                           c_pw_dids,
                           vcx_messages_download.cb)

    logger.debug("vcx_messages_download completed")
    return result

async def vcx_messages_update_status(msg_ids: str):
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_messages_update_status, "cb"):
        logger.debug("vcx_messages_update_status: Creating callback")
        vcx_messages_update_status.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_msg_ids = c_char_p(msg_ids.encode('utf-8'))

    result = await do_call('vcx_messages_update_status',
                           c_msg_ids,
                           vcx_messages_update_status.cb)

    logger.debug("vcx_messages_update_status completed")
    return result
