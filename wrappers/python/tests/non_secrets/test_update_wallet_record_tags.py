import pytest

from indy import IndyError
from indy.error import ErrorCode
from tests.non_secrets.common import *



@pytest.mark.asyncio
async def test_update_wallet_record_tags_works_for_twice(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, None)  # tags_empty)
    await check_record_field(wallet_handle, "tags", tags_empty)

    await non_secrets.update_wallet_record_tags(wallet_handle, type_, id1, tags1)
    await check_record_field(wallet_handle, "tags", tags1)

    await non_secrets.update_wallet_record_tags(wallet_handle, type_, id1, tags2)
    await check_record_field(wallet_handle, "tags", tags2)

    await non_secrets.update_wallet_record_tags(wallet_handle, type_, id1, tags2x)
    await check_record_field(wallet_handle, "tags", tags2x)

