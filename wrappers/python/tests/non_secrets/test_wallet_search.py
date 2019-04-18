import pytest

from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_wallet_search_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, None)
    await non_secrets.add_wallet_record(wallet_handle, type_, id2, value2, None)

    search_handle = await non_secrets.open_wallet_search(wallet_handle, type_, query_empty, options_full)

    search_records = json.loads(
        await non_secrets.fetch_wallet_search_next_records(wallet_handle, search_handle, 2))
    print('search records: {}'.format(search_records))

    await non_secrets.close_wallet_search(search_handle)
