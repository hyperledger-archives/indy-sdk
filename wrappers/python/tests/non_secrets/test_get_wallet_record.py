import operator
import pytest

from indy import IndyError
from indy.error import ErrorCode
from tests.non_secrets.common import *


@pytest.mark.asyncio
async def test_get_wallet_record_works_for_default_options(wallet_handle):
    # await non_secrets.add_wallet_record(wallet_handle, type_, id1, value1, tags1)

    record = json.loads(
        await non_secrets.get_wallet_record(wallet_handle, type_, id1, options_empty))


