import pytest

from indy_sdk import agent, IndyError
from indy_sdk.error import ErrorCode


@pytest.mark.asyncio
async def test_agent_close_connection_works_for_outgoing(listener_with_identity):
    listener_handle, wallet_handle, did = listener_with_identity

    connection_handle = await agent.agent_connect(0, wallet_handle, did, did)
    assert connection_handle is not None

    await agent.agent_close_connection(connection_handle)

    with pytest.raises(IndyError) as e:
        await agent.agent_send(connection_handle, "msg")
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_agent_close_connection_works_for_incoming(listener_with_identity):
    listener_handle, wallet_handle, did = listener_with_identity

    connection_handle = await agent.agent_connect(0, wallet_handle, did, did)
    assert connection_handle is not None

    event = await agent.agent_wait_for_event([listener_handle])  # type: agent.ConnectionEvent

    assert type(event) is agent.ConnectionEvent
    assert event.connection_handle is not None

    await agent.agent_close_connection(event.connection_handle)

    with pytest.raises(IndyError) as e:
        await agent.agent_send(event.connection_handle, "msg")
    assert ErrorCode.CommonInvalidStructure == e.value.error_code

    await agent.agent_close_connection(connection_handle)
