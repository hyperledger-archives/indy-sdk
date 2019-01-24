from .libindy import do_call, create_cb

from ctypes import *
from typing import Optional

import logging


async def pack_message(wallet_handle: int, message: str, recv_key_list: str, sender: str) -> str:
    """
    Packs a message (Experimental)

    Note to use DID keys with this function you can call did.key_for_did to get key id (verkey)
    for specific DID.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param message: the message to be authcrypted for multiple parties
    :param recv_key_list: a list in json format of receiver's verkeys
    :param sender: the sender's verkey as a string When None is used in this parameter, anoncrypt is used
    :return: a json string following the Auth or Anon AMES format
    """

    logger = logging.getLogger(__name__)
    logger.debug("pack_message: >>> wallet_handle: %r, message: %r, recv_key_list: %r, my_vk: %r",
                 wallet_handle,
                 message,
                 recv_key_list,
                 my_vk)

    if not hasattr(pack_message, "cb"):
        logger.debug("pack_message: Creating callback")
        pack_message.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_message = c_char_p(message.encode('utf-8'))
    c_recv_key_list = c_char_p(recv_key_list.encode('utf-8'))
    c_my_vk = c_char_p(my_vk.encode('utf-8'))

    auth_ames = await do_call('indy_pack_message',
                        c_wallet_handle,
                        c_message,
                        c_recv_key_list,
                        c_my_vk,
                        pack_message.cb)

    auth_ames.decode()
    logger.debug("pack_message: <<< res: %r", res)
    return res


async def unpack_message(wallet_handle: int, ames_json: str, my_vk: str) -> (str, str):
    """
    deserializes a AMES json string and decrypts the message returning the message and the sender's verkey if it was an AuthAMES. 
    If it is an AnonAMES it will deserialize, decrypt, and return the message with an empty sender_vk string.

    :param wallet_handle: wallet handler (created by open_wallet).
    :param ames_json: a json string serialized using either AuthAMES or AnonAMES
    :param my_vk: the verkey to authcrypt with
    :return: message: the unencrypted message
             sender_vk: the sender's verkey if AuthAMES, else an empty string
    """

    logger = logging.getLogger(__name__)
    logger.debug("unpack_message: >>> wallet_handle: %r, ames_json: %r, my_vk: %r",
                 wallet_handle,
                 ames_json,
                 my_vk)

    if not hasattr(unpack_message, "cb"):
        logger.debug("unpack_message: Creating callback")
        unpack_message.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_char_p))

    c_wallet_handle = c_int32(wallet_handle)
    c_ames_json = c_char_p(ames_json.encode('utf-8'))
    c_my_vk = c_char_p(my_vk.encode('utf-8'))

    message, sender_vk = await do_call('indy_unpack_messasge',
                                       c_wallet_handle,
                                       c_ames_json,
                                       c_my_vk,
                                       unpack_message.cb)

    res = (message.decode(), sender_vk.decode())
    logger.debug("unpack_message: <<< res: %r", res)
    return res