from indy import IndyError
from indy import crypto

import pytest

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_anon_crypt_works(verkey_my2, message):
    await crypto.anon_crypt(verkey_my2, message)


@pytest.mark.asyncio
async def test_anon_crypt_works_for_invalid_recipient_vk(message):
    with pytest.raises(IndyError) as e:
        await crypto.anon_crypt('invalidVerkeyLength', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code

    with pytest.raises(IndyError) as e:
        await crypto.anon_crypt('CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
