from ctypes import *
import logging
from vcx.common import do_call, create_cb


async def vcx_agent_provision(config: str) -> None:
    """
    Provision an agent in the agency, populate configuration and wallet for this agent.
    Example:
    import json
    enterprise_config = {
        'agency_url': 'http://localhost:8080',
        'agency_did': 'VsKV7grR1BUE29mG2Fm2kX',
        'agency_verkey': "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
        'wallet_name': 'LIBVCX_SDK_WALLET',
        'agent_seed': '00000000000000000000000001234561',
        'enterprise_seed': '000000000000000000000000Trustee1',
        'wallet_key': '1234'
    }
    vcx_config = await vcx_agent_provision(json.dumps(enterprise_config))
    :param config: JSON configuration
    :return: Configuration for vcx_init call.
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_agent_provision, "cb"):
        logger.debug("vcx_agent_provision: Creating callback")
        vcx_agent_provision.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))

    c_config = c_char_p(config.encode('utf-8'))

    result = await do_call('vcx_agent_provision_async',
                           c_config,
                           vcx_agent_provision.cb)

    logger.debug("vcx_agent_provision completed")
    return result.decode()

async def vcx_agent_update_info(config: str) -> None:
    """
    Update information on the agent (ie, comm method and type)
    :param config:
    :return:
    """
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
    """
    Get ledger fees from the sovrin network
    Example:
    fees = await vcx_ledger_get_fees()
    :return: JSON representing fees
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_ledger_get_fees, "cb"):
        logger.debug("vcx_ledger_get_fees: Creating callback")
        vcx_ledger_get_fees.cb = create_cb(CFUNCTYPE(None, c_uint32))

    result = await do_call('vcx_ledger_get_fees',
                           vcx_ledger_get_fees.cb)

    logger.debug("vcx_ledger_get_fees completed")
    return result

async def vcx_messages_download(status: str = None, uids: str = None, pw_dids: str = None) -> str:
    """
    Retrieve messages from the specified connection
    :param status:
    :param uids:
    :param pw_dids:
    :return:
    """
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


async def vcx_messages_update_status(msg_json: str):
    """
    Update the status of messages from the specified connection
    :param msg_json:
    :return:
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_messages_update_status, "cb"):
        logger.debug("vcx_messages_update_status: Creating callback")
        vcx_messages_update_status.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_msg_json = c_char_p(msg_json.encode('utf-8'))
    c_status = c_char_p("MS-106".encode('utf-8'))

    result = await do_call('vcx_messages_update_status',
                           c_status,
                           c_msg_json,
                           vcx_messages_update_status.cb)

    logger.debug("vcx_messages_update_status completed")
    return result
