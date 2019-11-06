import pytest

from indy import error
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_delete_wallet_record_tags_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await check_record_field(wallet_handle, "tags", tags1)

    await non_secrets.delete_wallet_record_tags(wallet_handle, type_, id1, '["tagName1"]')
    expected_tags = '{"tagName2": "5", "tagName3": "12"}'
    await check_record_field(wallet_handle, "tags", expected_tags)


@pytest.mark.asyncio
async def test_delete_wallet_record_tags_works_for_not_found_record(wallet_handle):
    with pytest.raises(error.WalletItemNotFound):
        await non_secrets.delete_wallet_record_tags(wallet_handle, type_, id1, '["tagName1", "tagName2", "tagName3"]')
