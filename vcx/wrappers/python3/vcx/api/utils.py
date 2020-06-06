import json
from ctypes import *
import logging
from typing import Optional

from vcx.common import do_call, create_cb, do_call_sync


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
        { "txnType1": amount1, "txnType2": amount2, ..., "txnTypeN": amountN }
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
    Retrieve messages from the agent
    :param status: optional, comma separated - query for messages with the specified status.
                                     Possible statuses:
                                     MS-101 - Created
                                     MS-102 - Sent
                                     MS-103 - Received
                                     MS-104 - Accepted
                                     MS-105 - Rejected
                                     MS-106 - Reviewed
    :param uids: optional, comma separated - query for messages with the specified uids
    :param pw_dids: optional, comma separated - DID's pointing to specific connection
    :return: message
        [{"pairwiseDID":"did","msgs":[{"statusCode":"MS-106","payload":null,"senderDID":"","uid":"6BDkgc3z0E","type":"aries","refMsgId":null,"deliveryDetails":[],"decryptedPayload":"{"@msg":".....","@type":{"fmt":"json","name":"aries","ver":"1.0"}}"}]}]
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
    :param msg_json: messages to update:
        [
            {
                "pairwiseDID":"string",
                "uids":["string"]
            },
            ...
        ]
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


def vcx_pool_set_handle(handle: int) -> None:
    """
    Sets the pool handle for libvcx to use, called before vcx_init_minimal
    :param handle: pool handle
    """
    c_handle = c_uint32(handle)

    do_call_sync('vcx_pool_set_handle', c_handle)


async def vcx_get_ledger_author_agreement():
    """
    Retrieve author agreement and acceptance mechanisms set on the Ledger
    :return: {"text":"Default agreement", "version":"1.0.0", "aml": {"label1": "description"}}
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_get_ledger_author_agreement, "cb"):
        logger.debug("vcx_get_ledger_author_agreement: Creating callback")
        vcx_get_ledger_author_agreement.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_char_p))
    result = await do_call('vcx_get_ledger_author_agreement',
                           vcx_get_ledger_author_agreement.cb)

    logger.debug("vcx_get_ledger_author_agreement completed")
    return result.decode()


def vcx_set_active_txn_author_agreement_meta(text: Optional[str],
                                             version: Optional[str],
                                             hash: Optional[str],
                                             acc_mech_type: str,
                                             time_of_acceptance: int) -> None:
    """
    Set some accepted agreement as active.
    As result of successful call of this function appropriate metadata will be appended to each write request.
    
    :param text and version - (optional) raw data about TAA from ledger.
               These parameters should be passed together.
               These parameters are required if hash parameter is ommited.
    :param hash - (optional) hash on text and version. This parameter is required if text and version parameters are ommited.
    :param acc_mech_type - mechanism how user has accepted the TAA
    :param time_of_acceptance - UTC timestamp when user has accepted the TAA

    :return: no value
    """
    logger = logging.getLogger(__name__)

    name = 'vcx_set_active_txn_author_agreement_meta'

    c_text = c_char_p(text.encode('utf-8')) if text else None
    c_version = c_char_p(version.encode('utf-8')) if version else None
    c_hash = c_char_p(hash.encode('utf-8')) if hash else None
    c_acc_mech_type = c_char_p(acc_mech_type.encode('utf-8'))
    c_time_of_acceptance = c_uint64(time_of_acceptance)

    do_call_sync(name, c_text, c_version, c_hash, c_acc_mech_type, c_time_of_acceptance)
    logger.debug("set_active_txn_author_agreement_meta completed")


async def vcx_get_request_price(action_json: str,
                                requester_info_json: Optional[str]):
    """
    Update the status of messages from the specified connection
    :param action_json: {
         "auth_type": ledger transaction alias or associated value,
         "auth_action": type of an action.,
         "field": transaction field,
         "old_value": (Optional) old value of a field, which can be changed to a new_value (mandatory for EDIT action),
         "new_value": (Optional) new value that can be used to fill the field,
     }
    :param requester_info_json: (Optional) {
         "role": string - role of a user which can sign transaction.
         "count": string - count of users.
         "is_owner": bool - if user is an owner of transaction.
     } otherwise context info will be used

    :return: price - tokens amount required for action performing
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_get_request_price, "cb"):
        logger.debug("vcx_get_request_price: Creating callback")
        vcx_get_request_price.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32, c_uint64))

    c_action_json = c_char_p(action_json.encode('utf-8'))
    c_requester_info_json = c_char_p(requester_info_json.encode('utf-8')) if requester_info_json is not None else None

    result = await do_call('vcx_get_request_price',
                           c_action_json,
                           c_requester_info_json,
                           vcx_get_request_price.cb)

    logger.debug("vcx_get_request_price completed")
    return result


async def vcx_endorse_transaction(transaction: str) -> None:
    """
    Endorse transaction to the ledger preserving an original author
    :param transaction: transaction to endorse
    :return:
    """
    logger = logging.getLogger(__name__)

    if not hasattr(vcx_endorse_transaction, "cb"):
        logger.debug("vcx_endorse_transaction: Creating callback")
        vcx_endorse_transaction.cb = create_cb(CFUNCTYPE(None, c_uint32, c_uint32))

    c_transaction = c_char_p(transaction.encode('utf-8'))

    result = await do_call('vcx_endorse_transaction',
                           c_transaction,
                           vcx_endorse_transaction.cb)

    logger.debug("vcx_endorse_transaction completed")
    return result

