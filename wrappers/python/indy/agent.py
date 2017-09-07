import asyncio
import logging
from ctypes import *
from typing import List, Tuple, Any

from .error import ErrorCode, IndyError
from .libindy import do_call, create_cb


class Event:
    """
    Base class for agent 2 agent communication events

    :handle: (int) Event source handle
    :error: (IndyError) If event is erroneous contains related IndyError exception
    """

    # handle: int
    # error: IndyError

    def __init__(self, handle: int, err: int):
        self.handle = handle
        self.error = IndyError(ErrorCode(err)) if err != ErrorCode.Success else None

    def is_success(self):
        """
        Checks is event erroneous or not

        :return: True if there are no errors assigned to this event and False otherwise
        """

        return self.error is None


class ConnectionEvent(Event):
    """
    Agent 2 agent communication listener's connection event

    :connection_handle: Incoming connection handle
    :sender_did: Sender DID
    :receiver_did: Receiver DID
    """
    # connection_handle: int
    # sender_did: str
    # receiver_did: str

    def __init__(self, handle: int, err: int, connection_handle: int, sender_did: bytes, receiver_did: bytes):
        logger = logging.getLogger(__name__)
        logger.debug("ConnectionEvent:__init__ >>> handle: %r, err: %r, connection_handle: %r, sender_did: %r, "
                     "receiver_did: %r",
                     handle,
                     err,
                     connection_handle,
                     sender_did,
                     receiver_did)

        super().__init__(handle, err)

        if self.is_success():
            self.connection_handle = connection_handle
            self.sender_did = sender_did.decode()
            self.receiver_did = receiver_did.decode()

        logger.debug("ConnectionEvent:__init__ <<< self: %r", self)


class MessageEvent(Event):
    """
    Agent 2 agent communication connection's message event

    :message: Incoming message
    """

    # message: str

    def __init__(self, handle: int, err: int, message: bytes):
        logger = logging.getLogger(__name__)
        logger.debug("MessageEvent:__init__ >>> handle: %r, err: %r, message: %r",
                     handle,
                     err,
                     message)

        super().__init__(handle, err)

        if self.is_success():
            self.message = message.decode()

        logger.debug("MessageEvent:__init__ <<< self: %r", self)


_events = []  # type: List[Event]
_event_waiters = []  # type: List[Tuple[List[int], Any, Any]]


def _notify_event_waiters():
    logger = logging.getLogger(__name__)
    logger.debug("_notify_event_waiters: >>> _event_waiters: %r, _events: %r",
                 _event_waiters,
                 _events)

    for i, (handles, event_loop, future) in enumerate(_event_waiters):
        for j, event in enumerate(_events):
            if event.handle in handles:
                del _event_waiters[i]
                del _events[j]
                event_loop.call_soon_threadsafe(lambda f, e: f.set_result(e),
                                                future,
                                                event)
                logger.debug("_notify_event_waiters: <<< handles: %r, event: %r", handles, event)
                return

    logger.debug("_notify_event_waiters: <<< no events")


async def agent_wait_for_event(handles: List[int]) -> Event:
    """
    Waits for events for listeners and connections defined by list of corresponded handles

    :param handles: list of listeners or connections handles
    :return: first occurred listener or connection event
    """

    logger = logging.getLogger(__name__)
    logger.debug("agent_wait_for_event: >>> handles: %r", handles)

    event_loop = asyncio.get_event_loop()
    future = event_loop.create_future()

    _event_waiters.append((handles, event_loop, future))
    _notify_event_waiters()

    res = await future

    logger.debug("agent_wait_for_event: <<< res: %r", res)
    return res


async def agent_connect(pool_handle: int,
                        wallet_handle: int,
                        sender_did: str,
                        receiver_did: str) -> int:
    """
    Establishes agent to agent connection.

    Information about sender Identity must be saved in the wallet with indy_create_and_store_my_did
    call before establishing of connection.

    Information about receiver Identity can be saved in the wallet with indy_store_their_did
    call before establishing of connection. If there is no corresponded wallet record for receiver Identity
    than this call will lookup Identity Ledger and cache this information in the wallet.

    Note that messages encryption/decryption will be performed automatically.

    After connection is established returned connection handle can be used to wait for messages with
    agent_wait_for_event or sending messages with agent_send.

    :param pool_handle: pool handle (created by open_pool_ledger).
    :param wallet_handle: wallet handle (created by open_wallet).
    :param sender_did: id of sender Identity stored in secured Wallet.
    :param receiver_did: id of receiver Identity.
    :return: connection handle to use for messages sending and waiting of incoming messages with agent_wait_for_event
    """

    logger = logging.getLogger(__name__)
    logger.debug("agent_connect: >>> pool_handle: %r, wallet_handle: %r, sender_did: %r, receiver_did: %r",
                 pool_handle,
                 wallet_handle,
                 sender_did,
                 receiver_did)

    if not hasattr(agent_connect, "connection_cb"):
        logger.debug("agent_connect: Creating connection callback")
        agent_connect.connection_cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    def _message_cb(connection_handle: int, err: int, message: bytes):
        logger.debug("agent_connect._message_cb: connection_handle: %r, err: %r, message: %r",
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

    logger.debug("agent_connect: <<< res: %r", res)
    return res


async def agent_listen(endpoint: str) -> int:
    """
    Starts listening of agent connections.

    Listener will accept only connections to registered DIDs by indy_agent_add_identity call.

    Information about sender Identity for incomming connection validation can be saved in the wallet
    with indy_store_their_did call before establishing of connection. If there is no corresponded
    wallet record for sender Identity than listener will lookup Identity Ledger and cache this
    information in the wallet.

    Note that messages encryption/decryption will be performed automatically.

    :param endpoint: endpoint to use in starting listener.
    :return: listener handle to use for waiting of incoming connections with agent_wait_for_event and management of
             assigned to this endpoint identities with agent_add_identity or agent_remove_identity.
    """

    logger = logging.getLogger(__name__)
    logger.debug("agent_listen: >>> endpoint: %r", endpoint)

    if not hasattr(agent_listen, "listener_cb"):
        logger.debug("agent_listen: Creating listener callback")
        agent_listen.listener_cb = create_cb(CFUNCTYPE(None, c_int32, c_int32, c_int32))

    def _connection_cb(listener_handle: int, err: int, connection_handle: int, sender_did: bytes, receiver_did: bytes):
        logger.debug("agent_connect._connection_cb: listener_handle: %r, err: %r, connection_handle: %r, sender_did: "
                     "%r, receiver_did: %r",
                     connection_handle,
                     err,
                     connection_handle,
                     sender_did,
                     receiver_did)
        _events.append(ConnectionEvent(listener_handle, err, connection_handle, sender_did, receiver_did))
        _notify_event_waiters()

    if not hasattr(agent_listen, "connection_cb"):
        logger.debug("agent_listen: Creating connection callback")
        agent_listen.connection_cb = CFUNCTYPE(None, c_int32, c_int32, c_int32, c_char_p, c_char_p)(_connection_cb)

    def _message_cb(connection_handle: int, err: int, message: bytes):
        logger.debug("agent_connect._message_cb: connection_handle: %r, err: %r, message: %r",
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

    logger.debug("agent_listen: <<< res: %r", res)
    return res


async def agent_add_identity(listener_handle: int,
                             pool_handle: int,
                             wallet_handle: int,
                             did: str) -> None:
    """
    Add identity to listener.

    Performs wallet lookup to find corresponded receiver Identity information.
    Information about receiver Identity must be saved in the wallet with
    indy_create_and_store_my_did call before this call.

    After successfully add_identity listener will start to accept incoming connection to added DID.

    :param listener_handle: listener handle (created by indy_agent_listen).
    :param pool_handle: pool handle (created by open_pool_ledger).
    :param wallet_handle: wallet handle (created by open_wallet).
    :param did: DID of identity.
    """
    logger = logging.getLogger(__name__)
    logger.debug("agent_add_identity: >>> listener_handle: %r, pool_handle: %r, wallet_handle: %r, did: %r",
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
                                wallet_handle: int,
                                did: str) -> None:
    """
    Remove identity from listener.

    Performs wallet lookup to find corresponded receiver Identity information.
    Information about receiver Identity must be saved in the wallet with
    signus.create_and_store_my_did call before this call.

    After successfully rm_identity listener will stop to accept incoming connection to removed DID.

    :param listener_handle: listener handle (created by indy_agent_listen).
    :param wallet_handle: wallet handle (created by open_wallet).
    :param did: DID of identity.
    """
    logger = logging.getLogger(__name__)
    logger.debug("agent_remove_identity: >>> listener_handle: %r, wallet_handle: %r, did: %r",
                 listener_handle,
                 wallet_handle,
                 did)

    if not hasattr(agent_remove_identity, "cb"):
        logger.debug("agent_remove_identity: Creating callback")
        agent_remove_identity.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_listener_handle = c_int32(listener_handle)
    c_wallet_handle = c_int32(wallet_handle)
    c_did = c_char_p(did.encode('utf-8'))

    await do_call('indy_agent_remove_identity',
                  c_listener_handle,
                  c_wallet_handle,
                  c_did,
                  agent_remove_identity.cb)

    logger.debug("agent_remove_identity: <<<")


async def agent_send(connection_handle: int, message: str) -> None:
    """"
    Sends message to connected agent.

    Note that this call works for both incoming and outgoing connections.
    Note that messages encryption/decryption will be performed automatically.

    :param connection_handle: connection handle returned by indy_agent_connect or indy_agent_listen calls.
    :param message: message to send.
    """

    logger = logging.getLogger(__name__)
    logger.debug("agent_send: >>> connection_handle: %r, message: %r",
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
    """
    Closes agent connection.

    Note that this call works for both incoming and outgoing connections.

    :param connection_handle: connection handle returned by indy_agent_connect or indy_agent_listen calls.
    """

    logger = logging.getLogger(__name__)
    logger.debug("agent_close_connection: >>> connection_handle: %r", connection_handle)

    if not hasattr(agent_close_connection, "cb"):
        logger.debug("agent_close_connection: Creating callback")
        agent_close_connection.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_connection_handle = c_int32(connection_handle)

    await do_call('indy_agent_close_connection',
                  c_connection_handle,
                  agent_close_connection.cb)

    logger.debug("agent_close_connection: <<<")


async def agent_close_listener(listener_handle: int) -> None:
    """
    Closes listener and stops listening for agent connections.

    Note that all opened incomming connections will be closed automatically.

    :param listener_handle: Listener handle returned by indy_agent_listen call.
    """

    logger = logging.getLogger(__name__)
    logger.debug("agent_close_listener: >>> listener_handle: %r", listener_handle)

    if not hasattr(agent_close_listener, "cb"):
        logger.debug("agent_close_listener: Creating callback")
        agent_close_listener.cb = create_cb(CFUNCTYPE(None, c_int32, c_int32))

    c_listener_handle = c_int32(listener_handle)

    await do_call('indy_agent_close_listener',
                  c_listener_handle,
                  agent_close_listener.cb)

    logger.debug("agent_close_listener: <<<")
