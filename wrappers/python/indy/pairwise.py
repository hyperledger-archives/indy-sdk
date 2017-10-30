from .libindy import do_call, create_cb

from ctypes import *
from typing import Optional

import logging


async def is_pairwise_exists(wallet_handle: int,
                             their_did: str) -> bool:
    """
    Check if pairwise is exists.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param their_did: encoded Did.
    :return: true - if pairwise is exists, false - otherwise
    """

    logger = logging.getLogger(__name__)
    logger.debug("is_pairwise_exists: >>> wallet_handle: %r, their_did: %r",
                 wallet_handle,
                 their_did)

    if not hasattr(is_pairwise_exists, "cb"):
        logger.debug("is_pairwise_exists: Creating callback")
        is_pairwise_exists.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_bool))

    c_wallet_handle = c_int32(wallet_handle)
    c_their_did = c_char_p(their_did.encode('utf-8'))

    res = await do_call('indy_is_pairwise_exists',
                        c_wallet_handle,
                        c_their_did,
                        is_pairwise_exists.cb)

    logger.debug("is_pairwise_exists: <<< res: %r", res)
    return res


async def create_pairwise(wallet_handle: int,
                          their_did: str,
                          my_did: str,
                          metadata: Optional[str]) -> None:
    """
    Creates pairwise.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param their_did: encrypting DID
    :param my_did: encrypting DID
    :param metadata: (Optional) extra information for pairwise
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("create_pairwise: >>> wallet_handle: %r, their_did: %r, my_did: %r, metadata: %r",
                 wallet_handle,
                 their_did,
                 my_did,
                 metadata)

    if not hasattr(create_pairwise, "cb"):
        logger.debug("create_pairwise: Creating callback")
        create_pairwise.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_their_did = c_char_p(their_did.encode('utf-8'))
    c_my_did = c_char_p(my_did.encode('utf-8'))
    c_metadata = c_char_p(metadata.encode('utf-8')) if metadata is not None else None

    await do_call('indy_create_pairwise',
                  c_wallet_handle,
                  c_their_did,
                  c_my_did,
                  c_metadata,
                  create_pairwise.cb)

    logger.debug("create_pairwise: <<<")


async def list_pairwise(wallet_handle: int) -> str:
    """
    Get list of saved pairwise.

    :param wallet_handle: wallet handler (created by open_wallet).
    :return: pairwise_list: list of saved pairwise
    """

    logger = logging.getLogger(__name__)
    logger.debug("list_pairwise: >>> wallet_handle: %r", wallet_handle)

    if not hasattr(list_pairwise, "cb"):
        logger.debug("list_pairwise: Creating callback")
        list_pairwise.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)

    pairwise_list = await do_call('indy_list_pairwise',
                                  c_wallet_handle,
                                  list_pairwise.cb)

    res = pairwise_list.decode()
    logger.debug("list_pairwise: <<< res: %r", res)
    return res


async def get_pairwise(wallet_handle: int,
                       their_did: str) -> None:
    """
    Gets pairwise information for specific their_did.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param their_did: encoded Did
    :return: pairwise_info_json: did info associated with their did
    """

    logger = logging.getLogger(__name__)
    logger.debug("get_pairwise: >>> wallet_handle: %r, their_did: %r",
                 wallet_handle,
                 their_did)

    if not hasattr(get_pairwise, "cb"):
        logger.debug("get_pairwise: Creating callback")
        get_pairwise.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_their_did = c_char_p(their_did.encode('utf-8'))

    pairwise_info_json = await do_call('indy_get_pairwise',
                                       c_wallet_handle,
                                       c_their_did,
                                       get_pairwise.cb)

    res = pairwise_info_json.decode()
    logger.debug("get_pairwise: <<< res: %r", res)
    return res


async def set_pairwise_metadata(wallet_handle: int,
                                their_did: str,
                                metadata: Optional[str]) -> None:
    """
    Save some data in the Wallet for pairwise associated with Did.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param their_did: encoded DID
    :param metadata: some extra information for pairwise
    :return: Error code
    """

    logger = logging.getLogger(__name__)
    logger.debug("set_pairwise_metadata: >>> wallet_handle: %r, their_did: %r, metadata: %r",
                 wallet_handle,
                 their_did,
                 metadata)

    if not hasattr(set_pairwise_metadata, "cb"):
        logger.debug("set_pairwise_metadata: Creating callback")
        set_pairwise_metadata.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_wallet_handle = c_int32(wallet_handle)
    c_their_did = c_char_p(their_did.encode('utf-8'))
    c_metadata = c_char_p(metadata.encode('utf-8')) if metadata is not None else None

    await do_call('indy_set_pairwise_metadata',
                  c_wallet_handle,
                  c_their_did,
                  c_metadata,
                  set_pairwise_metadata.cb)

    logger.debug("set_pairwise_metadata: <<<")
