from indy import ledger

import json
import pytest

@pytest.mark.asyncio
async def test_build_get_frozen_ledgers_request(did_trustee):
    response = json.loads(await ledger.build_get_frozen_ledgers_request(did_trustee))
