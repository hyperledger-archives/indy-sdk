import pytest

from indy import crypto, error


@pytest.mark.asyncio
async def test_anon_crypt_works(verkey_my2, message):
    await crypto.anon_crypt(verkey_my2, message)


@pytest.mark.asyncio
async def test_anon_crypt_works_for_invalid_recipient_vk(message):
    with pytest.raises(error.CommonInvalidStructure):
        await crypto.anon_crypt('invalidVerkeyLength', message)

    with pytest.raises(error.CommonInvalidStructure):
        await crypto.anon_crypt('CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
