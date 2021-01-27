from indy import ledger

import json
import pytest

@pytest.mark.asyncio
async def test_build_freeze_ledgers_request(did_trustee):
    ledgers_ids = '[6, 78, 75]'
    response = json.loads(await ledger.build_freeze_ledgers_request(did_trustee, ledgers_ids))
