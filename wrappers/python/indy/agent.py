import asyncio
import logging
from ctypes import *
from typing import List, Tuple, Any

from .error import ErrorCode, IndyError
from .libindy import do_call, create_cb


class Event:
    handle: int
    error: IndexError

    def is_success(self):
        return self.error is None


class ConnectionEvent(Event):
    connection_handle: int
    sender_did: str
    receiver_did: str

    def __init__(self, handle: int, err: int, connection_handle: int, sender_did: str, receiver_did):
        self.handle = handle,

        if err != ErrorCode.Success:
            self.error = IndyError(ErrorCode(err))
        else:
            self.connection_handle = connection_handle
            self.sender_did = sender_did
            self.receiver_did = receiver_did


class MessageEvent(Event):
    message: str

    def __init__(self, handle: int, err: int, message: str):
        self.handle = handle,

        if err != ErrorCode.Success:
            self._error = IndyError(ErrorCode(err))
        else:
            self.message = message


_events: List[Event] = []
_event_waiters: List[Tuple[List[int], Any, Any]] = []


def _notify_event_waiters():
    for i, (handles, event_loop, future) in enumerate(_event_waiters):
        for j, event in enumerate(_events):
            if event.handle in handles:
                del _event_waiters[i]
                del _events[j]
                event_loop.call_soon_threadsafe(lambda f, e: f.set_result(e),
                                                future,
                                                event)


async def agent_wait_for_event(handles: List[int]) -> Event:
    logger = logging.getLogger(__name__)
    logger.debug("agent_wait_for_event: >>> handles: %s", handles)

    event_loop = asyncio.get_event_loop()
    future = event_loop.create_future()

    _event_waiters.append((handles, event_loop, future))
    _notify_event_waiters()

    res = await future

    logger.debug("agent_wait_for_event: <<< res: %s", res)
    return res


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
        logger.debug("agent_connect: Creating connection callback")
        agent_connect.connection_cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    def _message_cb(connection_handle: int, err: int, message: str):
        logger.debug("agent_connect._message_cb: connection_handle: %i, err: %i, message: %s",
                     connection_handle,
                     err,
                     message)
        _events.append(MessageEvent(connection_handle, err, message))
        _notify_event_waiters()

    if not hasattr(agent_connect, "message_cb"):
        logger.debug("agent_connect: Creating message callback")
        agent_connect.message_cb = CFUNCTYPE(None, c_int32, c_int32, c_char_p)(_message_cb)

    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_sender_did = c_char_p(sender_did.encode('utf-8'))
    c_receiver_did = c_char_p(receiver_did.encode('utf-8'))

    res = await do_call('indy_agent_connect',
                        c_pool_handle,
                        c_wallet_handle,
                        c_sender_did,
                        c_receiver_did,
                        agent_connect.connection_cb,
                        agent_connect.message_cb)

    logger.debug("agent_connect: <<< res: %i", res)
    return res


async def agent_listen(endpoint: str) -> int:
    logger = logging.getLogger(__name__)
    logger.debug("agent_listen: >>> endpoint: %s", endpoint)

    if not hasattr(agent_listen, "listener_cb"):
        logger.debug("agent_listen: Creating listener callback")
        agent_listen.listener_cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    def _connection_cb(listener_handle: int, err: int, connection_handle: int, sender_did: str, receiver_did: str):
        logger.debug("agent_connect._connection_cb: listener_handle: %i, err: %i, connection_handle: %i, sender_did: "
                     "%s, receiver_did: %s",
                     connection_handle,
                     err,
                     connection_handle,
                     sender_did,
                     receiver_did)
        _events.append(ConnectionEvent(listener_handle, err, connection_handle, sender_did, receiver_did))
        _notify_event_waiters()

    if not hasattr(agent_listen, "connection_cb"):
        logger.debug("agent_listen: Creating connection callback")
        agent_listen.connection_cb = CFUNCTYPE(None, c_int32, c_int32, c_char_p)(_connection_cb)

    def _message_cb(connection_handle: int, err: int, message: str):
        logger.debug("agent_connect._message_cb: connection_handle: %i, err: %i, message: %s",
                     connection_handle,
                     err,
                     message)
        _events.append(MessageEvent(connection_handle, err, message))
        _notify_event_waiters()

    if not hasattr(agent_listen, "message_cb"):
        logger.debug("agent_connect: Creating message callback")
        agent_listen.message_cb = CFUNCTYPE(None, c_int32, c_int32, c_char_p)(_message_cb)

    c_endpoint = c_char_p(endpoint.encode('utf-8'))

    res = await do_call('indy_agent_listen',
                        c_endpoint,
                        agent_listen.listener_cb,
                        agent_listen.connection_cb,
                        agent_listen.message_cb)

    logger.debug("agent_listen: <<< res: %i", res)
    return res


async def agent_add_identity(listener_handle: int,
                             pool_handle: int,
                             wallet_handle: int,
                             did: str) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("agent_add_identity: >>> listener_handle: %i, pool_handle: %i, wallet_handle: %i, did: %s",
                 listener_handle,
                 pool_handle,
                 wallet_handle,
                 did)

    if not hasattr(agent_add_identity, "cb"):
        logger.debug("agent_add_identity: Creating callback")
        agent_add_identity.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_listener_handle = c_int32(listener_handle)
    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    await do_call('indy_agent_add_identity',
                  c_listener_handle,
                  c_pool_handle,
                  c_wallet_handle,
                  c_did,
                  agent_add_identity.cb)

    logger.debug("agent_add_identity: <<<")


async def agent_remove_identity(listener_handle: int,
                                pool_handle: int,
                                wallet_handle: int,
                                did: str) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("agent_remove_identity: >>> listener_handle: %i, pool_handle: %i, wallet_handle: %i, did: %s",
                 listener_handle,
                 pool_handle,
                 wallet_handle,
                 did)

    if not hasattr(agent_remove_identity, "cb"):
        logger.debug("agent_remove_identity: Creating callback")
        agent_remove_identity.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_listener_handle = c_int32(listener_handle)
    c_pool_handle = c_int32(pool_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    await do_call('indy_agent_remove_identity',
                  c_listener_handle,
                  c_pool_handle,
                  c_wallet_handle,
                  c_did,
                  agent_remove_identity.cb)

    logger.debug("agent_remove_identity: <<<")


async def agent_send(connection_handle: int, message: str) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("agent_send: >>> connection_handle: %i, message: %s",
                 connection_handle,
                 message)

    if not hasattr(agent_send, "cb"):
        logger.debug("agent_send: Creating callback")
        agent_send.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_connection_handle = c_int32(connection_handle)
    c_message = c_char_p(message.encode('utf-8'))

    await do_call('indy_agent_send',
                  c_connection_handle,
                  c_message,
                  agent_send.cb)

    logger.debug("agent_send: <<<")


async def agent_close_connection(connection_handle: int) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("agent_close_connection: >>> connection_handle: %i", connection_handle)

    if not hasattr(agent_close_connection, "cb"):
        logger.debug("agent_close_connection: Creating callback")
        agent_close_connection.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_connection_handle = c_int32(connection_handle)

    await do_call('indy_agent_close_connection',
                  c_connection_handle,
                  agent_close_connection.cb)

    logger.debug("agent_close_connection: <<<")


async def agent_close_listener(listener_handle: int) -> None:
    logger = logging.getLogger(__name__)
    logger.debug("agent_close_listener: >>> listener_handle: %i", listener_handle)

    if not hasattr(agent_close_listener, "cb"):
        logger.debug("agent_close_listener: Creating callback")
        agent_close_listener.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_listener_handle = c_int32(listener_handle)

    await do_call('indy_agent_close_listener',
                  c_listener_handle,
                  agent_close_listener.cb)

    logger.debug("agent_close_listener: <<<")
