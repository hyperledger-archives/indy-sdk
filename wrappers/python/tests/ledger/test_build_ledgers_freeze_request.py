from indy import ledger

import json
import pytest
from typing import List

@pytest.mark.asyncio
async def test_build_ledgers_freeze_request(did_trustee):
    ledgers_ids: List[int] = [0, 6, 78, 751]
    json.loads(await ledger.build_ledgers_freeze_request(did_trustee, ledgers_ids))

@pytest.mark.asyncio
async def test_build_ledgers_freeze_request_with_empty_data(did_trustee):
    ledgers_ids: List[int] = []
    json.loads(await ledger.build_ledgers_freeze_request(did_trustee, ledgers_ids))
