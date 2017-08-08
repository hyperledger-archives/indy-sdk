import pytest

from indy_sdk import IndyError
from indy_sdk import wallet
from indy.error import ErrorCode


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_handle_cleanup", [False])
async def test_close_wallet_works(wallet_name, wallet_handle):
    await wallet.close_wallet(wallet_handle)

    wallet_handle = await wallet.open_wallet(wallet_name, None, None)
    await wallet.close_wallet(wallet_handle)


@pytest.mark.asyncio
@pytest.mark.parametrize("wallet_handle_cleanup", [False])
async def test_close_wallet_works_for_twice(wallet_handle):
    with pytest.raises(IndyError) as e:
        await wallet.close_wallet(wallet_handle)
        await wallet.close_wallet(wallet_handle)

    assert ErrorCode.WalletInvalidHandle == e.value.error_code
