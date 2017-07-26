import json

import pytest

from indy import signus, ledger, agent
from tests.utils import wallet


@pytest.mark.asyncio
async def test_agent_send_works_for_all_data_in_wallet_present(connection):
    listener_handle, inc_con_handle, out_con_handle, wallet_handle, did = connection

    await agent.agent_send(out_con_handle, "msg_from_client")
    message_event = await agent.agent_wait_for_event([listener_handle, inc_con_handle])  # type: agent.MessageEvent

    assert type(message_event) is agent.MessageEvent
    assert message_event.handle == inc_con_handle
    assert message_event.message == "msg_from_client"

    await agent.agent_send(inc_con_handle, "msg_from_server")
    message_event = await agent.agent_wait_for_event([out_con_handle])  # type: agent.MessageEvent

    assert type(message_event) is agent.MessageEvent
    assert message_event.handle == out_con_handle
    assert message_event.message == "msg_from_server"
