import pytest

from indy import error
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_add_wallet_record_tags_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags_empty)
    await check_record_field(wallet_handle, "tags", tags_empty)

    await non_secrets.add_wallet_record_tags(wallet_handle, type_, id1, tags1)
    await check_record_field(wallet_handle, "tags", tags1)


@pytest.mark.asyncio
async def test_add_wallet_record_tags_works_for_not_found_record(wallet_handle):
    with pytest.raises(error.WalletItemNotFound):
        await non_secrets.add_wallet_record_tags(wallet_handle, type_, id1, tags1)
