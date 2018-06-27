import pytest

from indy import IndyError
from indy.error import ErrorCode
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_add_wallet_record_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)


@pytest.mark.asyncio
async def test_add_wallet_record_works_for_different_ids(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await non_secrets.add_wallet_record(wallet_handle, type_, id2, value1, tags1)
    await non_secrets.add_wallet_record(wallet_handle, type_, id3, value1, tags1)


@pytest.mark.asyncio
async def test_add_wallet_record_works_for_duplicate(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    with pytest.raises(IndyError) as e:
        await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    assert ErrorCode.WalletItemAlreadyExists == e.value.error_code
