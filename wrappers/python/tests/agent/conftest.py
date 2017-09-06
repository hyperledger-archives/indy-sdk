import json

import pytest

from indy import signus, agent


@pytest.fixture
async def wallet_with_identity(wallet_handle, endpoint):
    did, verkey, pk = await signus.create_and_store_my_did(wallet_handle, "{}")
    await signus.store_their_did(wallet_handle,
                                 json.dumps({
                                     "did": did,
                                     "verkey": verkey,
                                     "pk": pk,
                                     "endpoint": endpoint
                                 }))

    return wallet_handle, did


@pytest.fixture
async def wallet_with_identities(wallet_with_identity, endpoint):
    wallet_handle, did1 = wallet_with_identity

    did2, verkey2, pk2 = await signus.create_and_store_my_did(wallet_handle, "{}")
    await signus.store_their_did(wallet_handle,
                                 json.dumps({
                                     "did": did2,
                                     "verkey": verkey2,
                                     "pk": pk2,
                                     "endpoint": endpoint
                                 }))

    return wallet_handle, did1, did2


@pytest.fixture
def listener_handle(event_loop, endpoint):
    listener_handle = event_loop.run_until_complete(agent.agent_listen(endpoint))
    assert type(listener_handle) is int
    yield listener_handle
    event_loop.run_until_complete(agent.agent_close_listener(listener_handle))


@pytest.fixture
async def listener_with_identity(listener_handle, wallet_with_identity):
    wallet_handle, did = wallet_with_identity
    await agent.agent_add_identity(listener_handle, -1, wallet_handle, did)
    return listener_handle, wallet_handle, did


@pytest.fixture
async def listener_with_identities(listener_handle, wallet_with_identities):
    wallet_handle, did1, did2 = wallet_with_identities
    await agent.agent_add_identity(listener_handle, -1, wallet_handle, did1)
    await agent.agent_add_identity(listener_handle, -1, wallet_handle, did2)
    return listener_handle, wallet_handle, did1, did2


@pytest.fixture
def connection(event_loop, listener_with_identity):
    listener_handle, wallet_handle, did = listener_with_identity

    connection_handle = event_loop.run_until_complete(agent.agent_connect(0, wallet_handle, did, did))
    assert connection_handle is not None

    event = event_loop.run_until_complete(agent.agent_wait_for_event([listener_handle]))  # type: agent.ConnectionEvent

    assert type(event) is agent.ConnectionEvent
    assert event.handle == listener_handle
    assert event.sender_did == did
    assert event.receiver_did == did
    assert event.connection_handle is not None

    yield listener_handle, event.connection_handle, connection_handle, wallet_handle, did

    event_loop.run_until_complete(agent.agent_close_connection(event.connection_handle))
    event_loop.run_until_complete(agent.agent_close_connection(connection_handle))
