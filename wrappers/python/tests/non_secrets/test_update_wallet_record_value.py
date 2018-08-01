import pytest

from indy import IndyError
from indy.error import ErrorCode
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_update_wallet_record_value_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags_empty)
    await check_record_field(wallet_handle, "value", value1)

    await non_secrets.update_wallet_record_value(wallet_handle, type_, id1, value2)
    await check_record_field(wallet_handle, "value", value2)


@pytest.mark.asyncio
async def test_update_wallet_record_value_works_for_twice(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags_empty)
    await check_record_field(wallet_handle, "value", value1)

    await non_secrets.update_wallet_record_value(wallet_handle, type_, id1, value2)
    await check_record_field(wallet_handle, "value", value2)

    await non_secrets.update_wallet_record_value(wallet_handle, type_, id1, value3)
    await check_record_field(wallet_handle, "value", value3)


@pytest.mark.asyncio
async def test_update_wallet_record_value_works_for_not_found_record(wallet_handle):
    with pytest.raises(IndyError) as e:
        await non_secrets.update_wallet_record_value(wallet_handle, type_, id1, value1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code
