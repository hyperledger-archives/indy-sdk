import operator
import pytest

from indy import IndyError
from indy.error import ErrorCode
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_get_wallet_record_works_for_default_options(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)

    record = json.loads(
        await non_secrets.get_wallet_record(wallet_handle, type_, id1, options_empty))

    expected = {'id': id1, 'value': value1, 'tags': None, 'type': None}

    assert expected == record


@pytest.mark.asyncio
async def test_get_wallet_record_works_for_full_data(wallet_handle):
    await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)

    options_json = json.dumps({
        'retrieveType': True,
        'retrieveValue': True,
        'retrieveTags': True
    })

    record = json.loads(
        await non_secrets.get_wallet_record(wallet_handle, type_, id1, options_json))

    expected = {'id': id1, 'value': value1, 'tags': json.loads(tags1), 'type': type_}

    assert record == expected


@pytest.mark.asyncio
async def test_get_wallet_record_works_for_not_found_record(wallet_handle):
    with pytest.raises(IndyError) as e:
        await non_secrets.get_wallet_record(wallet_handle, type_, id1, options_empty)
    assert ErrorCode.WalletItemNotFound == e.value.error_code
