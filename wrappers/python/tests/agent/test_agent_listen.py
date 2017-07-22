import json
import logging

import pytest

from indy import signus, agent
from ..utils import storage, wallet

logging.basicConfig(level=logging.DEBUG)

endpoint = "127.0.0.1:9701"


@pytest.fixture(autouse=True)
def cleanup_storage():
    storage.cleanup()
    yield
    storage.cleanup()


# noinspection PyUnusedLocal
@pytest.fixture
async def wallet_with_identities(cleanup_storage):
    wallet_handle = await wallet.create_and_open_wallet()
    did, verkey, pk = await signus.create_and_store_my_did(wallet_handle, "{}")
    await signus.store_their_did(wallet_handle, json.dumps({
        "did": did,
        "verkey": verkey,
        "pk": pk,
        "endpoint": endpoint}))

    yield wallet_handle, did

    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
async def test_agent_listen_works(wallet_with_identities):
    wallet_handle, did = wallet_with_identities

    listener_handle = await agent.agent_listen(endpoint)
    assert listener_handle is not None

    await agent.agent_add_identity(listener_handle, -1, wallet_handle, did)

    connection_handle = await agent.agent_connect(0, wallet_handle, did, did)
    assert connection_handle is not None

    event = await agent.agent_wait_for_event([listener_handle, connection_handle])  # type: agent.ConnectionEvent

    assert type(event) is agent.ConnectionEvent
    assert event.handle == listener_handle
    assert event.sender_did == did
    assert event.receiver_did == did
    assert event.connection_handle is not None

    await agent.agent_close_connection(event.connection_handle)
    await agent.agent_close_connection(connection_handle)
    await agent.agent_close_listener(listener_handle)
