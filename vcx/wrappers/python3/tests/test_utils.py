import pytest
from vcx.error import ErrorCode, VcxError, error_message
from vcx.common import get_version, update_institution_info
from vcx.api.utils import vcx_agent_provision, vcx_agent_update_info, vcx_messages_download, vcx_messages_update_status, \
    vcx_get_ledger_author_agreement, vcx_set_active_txn_author_agreement_meta
from ctypes import *

provisionString = "{\"agency_url\":\"http://localhost:8080\",\"agency_did\":\"VsKV7grR1BUE29mG2Fm2kX\",\"agency_verkey\":\"Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR\",\"wallet_name\":\"test_provision_agent\",\"agent_seed\":null,\"enterprise_seed\":null,\"wallet_key\":\"123\"}"
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
async def test_update_agent_info_fails(cleanup):
    with pytest.raises(VcxError) as e:
        await vcx_agent_update_info("")
    assert ErrorCode.InvalidOption == e.value.error_code
    cleanup(True)


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
async def test_update_agent_info(cleanup):
    await vcx_agent_update_info(agentUpdateString)
    cleanup(True)


def test_get_version():
    assert get_version()


def test_update_institution_info(cleanup):
    # Returns None if successful and throws error otherwise
    assert update_institution_info('new name', 'new logo') is None
    cleanup(True)


@pytest.mark.asyncio
@pytest.mark.usefixtures('vcx_init_test_mode')
async def test_vcx_get_ledger_author_agreement():
    agreement = await vcx_get_ledger_author_agreement()
    assert agreement == '{"text":"Default indy agreement", "version":"1.0.0", "aml": {"acceptance mechanism label1": "description"}}'


def test_set_active_txn_author_agreement_meta():
    vcx_set_active_txn_author_agreement_meta('vcx agreement',
                                             '1.0.0',
                                             None,
                                             'acceptance type 1',
                                             123456789)