from indy import ledger

import json
import pytest


@pytest.mark.asyncio
async def test_build_get_txn_request_works(did_trustee):
    seq_no = 1
    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "3",
            "data": 1,
            "ledgerId": 1
        }
    }

    response = json.loads(await ledger.build_get_txn_request(did_trustee, None, seq_no))
    assert expected_response.items() <= response.items()


@pytest.mark.asyncio
async def test_build_get_txn_request_works_for_ledger_type(did_trustee):
    seq_no = 1
    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "3",
            "data": 1,
            "ledgerId": 0
        }
    }

    response = json.loads(await ledger.build_get_txn_request(did_trustee, 'POOL', seq_no))
    assert expected_response.items() <= response.items()
