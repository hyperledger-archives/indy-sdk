import json

import pytest

from indy_sdk import signus, ledger, agent
from tests.utils import wallet


@pytest.mark.asyncio
async def test_agent_connect_works_for_remote_data(endpoint,
                                                   pool_handle,
                                                   trustee1_seed):
    listener_wallet_handle = await wallet.create_and_open_wallet(wallet_name="listener_wallet")
    trustee_wallet_handle = await wallet.create_and_open_wallet(wallet_name="trustee_wallet")

    listener_did, listener_verkey, listener_pk = await signus.create_and_store_my_did(listener_wallet_handle, "{}")

    trustee_did, trustee_verkey, trustee_pk = await signus.create_and_store_my_did(
        trustee_wallet_handle,
        json.dumps({
            "seed": trustee1_seed
        }))

    nym_request = await ledger.build_nym_request(trustee_did, listener_did, listener_verkey, None, None)
    await ledger.sign_and_submit_request(pool_handle, trustee_wallet_handle, trustee_did, nym_request)

    attrib_request = await ledger.build_attrib_request(
        listener_did,
        listener_did,
        None,
        json.dumps({
            "endpoint": {
                "ha": endpoint,
                "verkey": listener_pk
            }
        }),
        None)
    await ledger.sign_and_submit_request(pool_handle, listener_wallet_handle, listener_did, attrib_request)

    listener_handle = await agent.agent_listen(endpoint)
    await agent.agent_add_identity(listener_handle, pool_handle, listener_wallet_handle, listener_did)

    sender_did = trustee_did
    sender_wallet_handle = trustee_wallet_handle
    connection_handle = await agent.agent_connect(pool_handle,
                                                  sender_wallet_handle,
                                                  sender_did,
                                                  listener_did)
    assert connection_handle is not None

    connection_event = await agent.agent_wait_for_event([listener_handle])  # type: agent.ConnectionEvent

    assert type(connection_event) is agent.ConnectionEvent
    assert connection_event.handle == listener_handle
    assert connection_event.sender_did == sender_did
    assert connection_event.receiver_did == listener_did
    assert connection_event.connection_handle is not None

    await agent.agent_close_connection(connection_handle)
    await agent.agent_close_connection(connection_event.connection_handle)
    await agent.agent_close_listener(listener_handle)
    await wallet.close_wallet(listener_wallet_handle)
    await wallet.close_wallet(trustee_wallet_handle)


@pytest.mark.asyncio
async def test_agent_connect_works_for_all_data_in_wallet_present(connection):
    assert connection is not None
