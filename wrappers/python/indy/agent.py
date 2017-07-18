import logging
from ctypes import *

from .libindy import do_call, create_cb


async def agent_connect(pool_handle: int,
                        wallet_handle: int,
                        sender_did: str,
                        receiver_did: str) -> int:
    logger = logging.getLogger(__name__)
    logger.debug("agent_connect: >>> pool_handle: %i, wallet_handle: %i, sender_did: %s, receiver_did: %s",
                 pool_handle,
                 wallet_handle,
                 sender_did,
                 receiver_did)

    if not hasattr(agent_connect, "connection_cb"):
        logger.debug("agent_connect: Creating callback")
        agent_connect.connection_cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    def _message_cb(connection_handle, err, message):
        logger.debug("agent_connect._message_cb: connection_handle: %i, err: %i, message: %s",
                     connection_handle,
                     err,
                     message)

    if not hasattr(agent_connect, "message_cb"):
        logger.debug("agent_connect: Creating connection callback")
        agent_connect.message_cb = CFUNCTYPE(None, c_int32, c_int32, c_int32)(_message_cb)

    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_sender_did = c_char_p(sender_did.encode('utf-8'))
    c_receiver_did = c_char_p(receiver_did.encode('utf-8'))

    res = await do_call('agent_connect',
                        c_pool_handle,
                        c_wallet_handle,
                        c_sender_did,
                        c_receiver_did,
                        agent_connect.connection_cb,
                        agent_connect.message_cb)

    logger.debug("agent_connect: <<< res: %i", res)
    return res
