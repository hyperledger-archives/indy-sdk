import pytest

from indy import error
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_add_wallet_record_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)


@pytest.mark.asyncio
async def test_add_wallet_record_works_for_duplicate(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    with pytest.raises(error.WalletItemAlreadyExists):
        await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
