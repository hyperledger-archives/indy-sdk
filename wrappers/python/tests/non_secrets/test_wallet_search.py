import pytest

from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_wallet_search_works(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await non_secrets.add_wallet_record(wallet_handle, type_, id2, value2, tags2)

    search_handle = await non_secrets.open_wallet_search(wallet_handle, type_, query_empty, options_empty)

    search_records = json.loads(
        await non_secrets.fetch_wallet_search_next_records(wallet_handle, search_handle, 2))

    assert {'id': id1, 'value': value1, 'tags': None, 'type': None} in search_records['records']
    assert {'id': id2, 'value': value2, 'tags': None, 'type': None} in search_records['records']

    await non_secrets.close_wallet_search(search_handle)


@pytest.mark.asyncio
async def test_wallet_search_works_for_options(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await non_secrets.add_wallet_record(wallet_handle, type_, id2, value2, tags2)

    options_json = json.dumps({
        'retrieveRecords': True,
        'retrieveTotalCount': False,
        'retrieveType': False,
        'retrieveValue': True,
        'retrieveTags': False
    })
    search_handle = await non_secrets.open_wallet_search(wallet_handle, type_, query_empty, options_json)

    search_records = json.loads(
        await non_secrets.fetch_wallet_search_next_records(wallet_handle, search_handle, 2))

    assert {'id': id1, 'value': value1, 'tags': None, 'type': None} in search_records['records']
    assert {'id': id2, 'value': value2, 'tags': None, 'type': None} in search_records['records']

    await non_secrets.close_wallet_search(search_handle)


@pytest.mark.asyncio
async def test_wallet_search_works_for_query(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)
    await non_secrets.add_wallet_record(wallet_handle, type_, id2, value2, tags2)

    query_json = json.dumps({
        'tagName1': 'str2'
    })
    search_handle = await non_secrets.open_wallet_search(wallet_handle, type_, query_json, options_empty)

    search_records = json.loads(
        await non_secrets.fetch_wallet_search_next_records(wallet_handle, search_handle, 2))

    assert {'id': id2, 'value': value2, 'tags': None, 'type': None} in search_records['records']

    await non_secrets.close_wallet_search(search_handle)
