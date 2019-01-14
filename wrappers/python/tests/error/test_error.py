import json

from indy import IndyError
from indy import crypto, did
from indy.libindy import set_runtime_config

import pytest

from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_error(wallet_handle, identity_trustee1, verkey_my2, message):
    (_, key) = identity_trustee1

    invalid_key = crypto.auth_crypt(wallet_handle, key, 'CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW', message)
    invalid_wallet_handle = crypto.auth_crypt(wallet_handle + 1, key, verkey_my2, message)
    empty_key = crypto.auth_crypt(wallet_handle, "", verkey_my2, message)

    with pytest.raises(IndyError) as e1:
        await invalid_key
    assert ErrorCode.CommonInvalidStructure == e1.value.error_code
    assert e1.value.message

    with pytest.raises(IndyError) as e2:
        await empty_key
    assert ErrorCode.CommonInvalidParam3 == e2.value.error_code
    assert e2.value.message

    with pytest.raises(IndyError) as e3:
        await invalid_wallet_handle
    assert ErrorCode.WalletInvalidHandle == e3.value.error_code
    assert e3.value.message
