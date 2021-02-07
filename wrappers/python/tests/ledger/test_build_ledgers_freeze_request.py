from indy import ledger
from typing import List
from indy.error import CommonInvalidStructure

import json
import pytest

@pytest.mark.asyncio
async def test_build_ledgers_freeze_request(did_trustee):
    ledgers_ids: List[int] = [0, 6, 78, 751]
    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "9",
            'ledgers_ids': ledgers_ids
        }
    }
    response = json.loads(await ledger.build_ledgers_freeze_request(did_trustee, ledgers_ids))
    assert expected_response.items() <= response.items()

@pytest.mark.asyncio
async def test_build_ledgers_freeze_request_with_empty_data(did_trustee):
    ledgers_ids: List[int] = []
    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "9",
            'ledgers_ids': ledgers_ids
        }
    }

    response = json.loads(await ledger.build_ledgers_freeze_request(did_trustee, ledgers_ids))
    assert expected_response.items() <= response.items()

@pytest.mark.asyncio
async def test_build_ledgers_freeze_request_with_str_data(did_trustee):
    ledgers_ids = ["0", "6", "78", "751"]

    with pytest.raises(CommonInvalidStructure):
        json.loads(await ledger.build_ledgers_freeze_request(did_trustee, ledgers_ids))

@pytest.mark.asyncio
async def test_build_ledgers_freeze_request_with_no_data(did_trustee):
    ledgers_ids = None

    with pytest.raises(CommonInvalidStructure):
        json.loads(await ledger.build_ledgers_freeze_request(did_trustee, ledgers_ids))
