import json

import pytest

from indy import crypto, did, error
from indy.libindy import set_runtime_config


@pytest.mark.asyncio
async def test_error(wallet_handle, identity_trustee1, verkey_my2, message):
    (_, key) = identity_trustee1

    invalid_key = crypto.auth_crypt(wallet_handle, key, 'CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    invalid_wallet_handle = crypto.auth_crypt(wallet_handle + 1, key, verkey_my2, message)
    empty_key = crypto.auth_crypt(wallet_handle, "", verkey_my2, message)

    with pytest.raises(error.CommonInvalidStructure) as e1:
        await invalid_key
    assert e1.value.message

    with pytest.raises(error.CommonInvalidParam3) as e2:
        await empty_key
    assert e2.value.message

    with pytest.raises(error.WalletInvalidHandle) as e3:
        await invalid_wallet_handle
    assert e3.value.message
