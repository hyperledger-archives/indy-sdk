import pytest

from indy import error
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_delete_wallet_record_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await non_secrets.delete_wallet_record(wallet_handle, type_, id1)
    with pytest.raises(error.WalletItemNotFound):
        await non_secrets.get_wallet_record(wallet_handle, type_, id1, "{}")


@pytest.mark.asyncio
async def test_delete_wallet_record_works_for_not_found_record(wallet_handle):
    with pytest.raises(error.WalletItemNotFound):
        await non_secrets.delete_wallet_record(wallet_handle, type_, id1)
