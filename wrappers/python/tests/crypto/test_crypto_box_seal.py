import pytest

from indy import IndyError, crypto
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_crypto_box_seal_works(verkey_my1, message):
    await crypto.crypto_box_seal(verkey_my1, message)


@pytest.mark.asyncio
async def test_crypto_box_seal_works_for_invalid_key(message):
    with pytest.raises(IndyError) as e:
        await crypto.crypto_box_seal('CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    assert ErrorCode.CommonInvalidStructure == e.value.error_code
