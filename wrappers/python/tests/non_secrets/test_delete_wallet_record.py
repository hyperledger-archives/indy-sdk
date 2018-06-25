
import pytest

from indy import IndyError
from indy.error import ErrorCode
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_delete_wallet_record_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await non_secrets.delete_wallet_record(wallet_handle, type_, id1)
    with pytest.raises(IndyError) as e:
        await non_secrets.get_wallet_record(wallet_handle, type_, id1, "{}")
    assert ErrorCode.WalletItemNotFound == e.value.error_code

@pytest.mark.asyncio
async def test_delete_wallet_record_works_for_twice(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await non_secrets.delete_wallet_record(wallet_handle, type_, id1)

    with pytest.raises(IndyError) as e:
        await non_secrets.delete_wallet_record(wallet_handle, type_, id1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code


@pytest.mark.asyncio
async def test_delete_wallet_record_works_for_not_found_record(wallet_handle):
    with pytest.raises(IndyError) as e:
        await non_secrets.delete_wallet_record(wallet_handle, type_, id1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code
