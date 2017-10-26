import logging
from ctypes import *

from .libindy import do_call, create_cb


async def prep_msg(wallet_handle: int,
                   sender_vk: str,
                   recipient_vk: str,
                   msg: bytes) -> bytes:
    """
    :param wallet_handle: wallet handler (created by open_wallet).
    :param sender_vk: DID
    :param recipient_vk: DID
    :param msg: a message to be prepared
    :return: prepared message
    """

    logger = logging.getLogger(__name__)
    logger.debug("prep_msg: >>> wallet_handle: %r, sender_vk: %r, recipient_vk: %r, msg: %r",
                 wallet_handle,
                 sender_vk,
                 recipient_vk,
                 msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len]),

    if not hasattr(prep_msg, "cb"):
        logger.debug("prep_msg: Creating callback")
        prep_msg.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_sender_vk = c_char_p(sender_vk.encode('utf-8'))
    c_recipient_vk = c_char_p(recipient_vk.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))

    encrypted_msg = await do_call('indy_prep_msg',
                                  c_wallet_handle,
                                  c_sender_vk,
                                  c_recipient_vk,
                                  msg,
                                  c_msg_len,
                                  prep_msg.cb)

    logger.debug("prep_msg: <<< res: %r", encrypted_msg)
    return encrypted_msg


async def prep_anonymous_msg(recipient_vk: str,
                             msg: bytes) -> bytes:
    """
    :param recipient_vk: DID
    :param msg: a message to be prepared
    :return: prepared message
    """

    logger = logging.getLogger(__name__)
    logger.debug("prep_anonymous_msg: >>> recipient_vk: %r, msg: %r",
                 recipient_vk,
                 msg)

    def transform_cb(arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return bytes(arr_ptr[:arr_len]),

    if not hasattr(prep_anonymous_msg, "cb"):
        logger.debug("prep_anonymous_msg: Creating callback")
        prep_anonymous_msg.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, POINTER(c_uint8), c_uint32), transform_cb)

    c_recipient_vk = c_char_p(recipient_vk.encode('utf-8'))
    c_msg_len = c_uint32(len(msg))

    encrypted_msg = await do_call('indy_prep_anonymous_msg',
                                  c_recipient_vk,
                                  msg,
                                  c_msg_len,
                                  prep_anonymous_msg.cb)

    logger.debug("prep_anonymous_msg: <<< res: %r", encrypted_msg)
    return encrypted_msg


async def parse_msg(wallet_handle: int,
                    recipient_vk: str,
                    encrypted_msg: bytes) -> (str, bytes):
    """
    :param wallet_handle: wallet handler (created by open_wallet).
    :param recipient_vk: DID
    :param encrypted_msg: an encrypted message
    :return: (sender_vk, message)
    """

    logger = logging.getLogger(__name__)
    logger.debug("parse_msg: >>> wallet_handle: %r, recipient_vk: %r, encrypted_msg: %r",
                 wallet_handle,
                 recipient_vk,
                 encrypted_msg)

    def transform_cb(key: c_char_p, arr_ptr: POINTER(c_uint8), arr_len: c_uint32):
        return (key, bytes(arr_ptr[:arr_len])),

    if not hasattr(parse_msg, "cb"):
        logger.debug("parse_msg: Creating callback")
        parse_msg.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p, POINTER(c_uint8), c_uint32), transform_cb)

    c_wallet_handle = c_int32(wallet_handle)
    c_recipient_vk = c_char_p(recipient_vk.encode('utf-8'))
    c_encrypted_msg_len = c_uint32(len(encrypted_msg))

    (sender_vk, msg) = await do_call('indy_parse_msg',
                                     c_wallet_handle,
                                     c_recipient_vk,
                                     bytes(encrypted_msg),
                                     c_encrypted_msg_len,
                                     parse_msg.cb)

    if sender_vk:
        sender_vk = sender_vk.decode()

    logger.debug("parse_msg: <<< res: %r", (sender_vk, msg))
    return sender_vk, msg
