import pytest

from indy import IndyError
from indy.error import ErrorCode
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_add_wallet_record_tags_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags_empty)
    await check_record_field(wallet_handle, "tags", tags_empty)

    await non_secrets.add_wallet_record_tags(wallet_handle, type_, id1, tags1)
    await check_record_field(wallet_handle, "tags", tags1)


@pytest.mark.asyncio
async def test_add_wallet_record_tags_works_for_twice(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags_empty)
    await check_record_field(wallet_handle, "tags", tags_empty)

    tags1 = '{"tagName1": "str1"}'
    await non_secrets.add_wallet_record_tags(wallet_handle, type_, id1, tags1)
    await check_record_field(wallet_handle, "tags", tags1)

    tags2 = '{"tagName2": "str2"}'
    await non_secrets.add_wallet_record_tags(wallet_handle, type_, id1, tags2)

    expected_tags = '{"tagName1": "str1", "tagName2": "str2"}'
    await check_record_field(wallet_handle, "tags", expected_tags)


@pytest.mark.asyncio
async def test_add_wallet_record_tags_works_for_not_found_record(wallet_handle):
    with pytest.raises(IndyError) as e:
        await non_secrets.add_wallet_record_tags(wallet_handle, type_, id1, tags1)
    assert ErrorCode.WalletItemNotFound == e.value.error_code
