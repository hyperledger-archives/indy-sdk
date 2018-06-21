import os

import pytest

from indy import IndyError
from indy import wallet
from indy.error import ErrorCode


@pytest.mark.asyncio
async def test_export_wallet_works(wallet_handle, export_config, export_path):
    await wallet.export_wallet(wallet_handle, export_config)
    os.path.exists(export_path)


@pytest.mark.asyncio
async def test_export_wallet_works_for_path_exists(wallet_handle, export_config, export_path):
    os.makedirs(export_path, exist_ok=True)
    os.path.exists(export_path)

    with pytest.raises(IndyError) as e:
        await wallet.export_wallet(wallet_handle, export_config)

    assert ErrorCode.CommonIOError == e.value.error_code
