import pytest
from vcx.error import ErrorCode, VcxError
from vcx.common import error_message, get_version, update_institution_info
from vcx.api.utils import vcx_agent_provision, vcx_agent_update_info, vcx_messages_download, vcx_messages_update_status
from ctypes import *

provisionString = "{\"agency_url\":\"https://enym-eagency.pdev.evernym.com\",\"agency_did\":\"Ab8TvZa3Q19VNkQVzAWVL7\",\"agency_verkey\":\"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf\",\"wallet_name\":\"test_provision_agent\",\"agent_seed\":null,\"enterprise_seed\":null,\"wallet_key\":\"123\"}"
agentUpdateString = "{\"id\":\"123\",\"value\":\"value\"}"
updateMessagesString = "[{\"pairwiseDID\":\"QSrw8hebcvQxiwBETmAaRs\",\"uids\":[\"mgrmngq\"]}]"


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_provision_agent_fails():
    with pytest.raises(VcxError) as e:
        await vcx_agent_provision("")
    assert ErrorCode.InvalidOption == e.value.error_code


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_provision_agent():
    config = await vcx_agent_provision(provisionString)
    assert config


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_agent_info_fails():
    with pytest.raises(VcxError) as e:
        await vcx_agent_update_info("")
    assert ErrorCode.InvalidOption == e.value.error_code

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_vcx_messages_download():
    messages = await vcx_messages_download()
    assert messages

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_vcx_messages_update_status():
    await vcx_messages_update_status(updateMessagesString)

@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_update_agent_info():
    await vcx_agent_update_info(agentUpdateString)


def test_get_version():
    assert get_version()


def test_update_institution_info():
    # Returns None if successful and throws error otherwise
    assert update_institution_info('new name', 'new logo') is None
