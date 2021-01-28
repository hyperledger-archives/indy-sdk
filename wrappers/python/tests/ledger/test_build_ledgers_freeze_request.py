from indy import ledger

import json
import pytest

@pytest.mark.asyncio
async def test_build_ledgers_freeze_request(did_trustee):
    ledgers_ids = '[6, 78, 75]'
    response = json.loads(await ledger.build_ledgers_freeze_request(did_trustee, ledgers_ids))
