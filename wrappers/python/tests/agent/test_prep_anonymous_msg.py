from indy import IndyError
from indy import agent

import pytest

from indy.error import ErrorCode
from tests.agent.conftest import check_message


@pytest.mark.asyncio
async def test_prep_msg_works_for_created_key(verkey_my2, message):
    encrypted_msg = await agent.prep_anonymous_msg(verkey_my2, message)
    await check_message(encrypted_msg, message)


@pytest.mark.asyncio
async def test_prep_anonymous_msg_works_for_invalid_recipient_vk(message):
    with pytest.raises(IndyError) as e:
        await agent.prep_anonymous_msg('invalidVerkeyLength', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code

    with pytest.raises(IndyError) as e:
        await agent.prep_anonymous_msg('CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
