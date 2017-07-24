import pytest

from indy import agent, IndyError
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_agent_close_connection_works_for_outgoing(listener_with_identity):
    listener_handle, wallet_handle, did = listener_with_identity

    connection_handle = await agent.agent_connect(0, wallet_handle, did, did)
    assert connection_handle is not None

    await agent.agent_close_connection(connection_handle)

    try:
        await agent.agent_send(connection_handle, "msg")
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.CommonInvalidStructure)) == type(e) \
               and IndyError(ErrorCode.CommonInvalidStructure).args == e.args


@pytest.mark.asyncio
async def test_agent_close_connection_works_for_incoming(listener_with_identity):
    listener_handle, wallet_handle, did = listener_with_identity

    connection_handle = await agent.agent_connect(0, wallet_handle, did, did)
    assert connection_handle is not None

    event = await agent.agent_wait_for_event([listener_handle])  # type: agent.ConnectionEvent

    assert type(event) is agent.ConnectionEvent
    assert event.connection_handle is not None

    await agent.agent_close_connection(event.connection_handle)

    try:
        await agent.agent_send(event.connection_handle, "msg")
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.CommonInvalidStructure)) == type(e) \
               and IndyError(ErrorCode.CommonInvalidStructure).args == e.args
    finally:
        await agent.agent_close_connection(connection_handle)
