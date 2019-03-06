from indy import ledger
from indy.error import *

import json
import pytest

identifier = "Th7MpTaRZVRYnPiabds81Y"
txn_type = "NYM"
add_auth_action = "ADD"
edit_auth_action = "EDIT"
field = 'role'
old_value = '0'
new_value = '101'
constraint = {
    "sig_count": 1,
    "metadata": {},
    "role": "0",
    "constraint_id": "ROLE",
    "need_to_be_owner": False
}


@pytest.mark.asyncio
async def test_build_auth_rule_request_works_for_add_auth_action():
    expected_request = {
        "identifier": identifier,
        "operation": {
            "type": "120",
            "auth_type": "1",
            "auth_action": add_auth_action,
            "field": field,
            "new_value": new_value,
            "constraint": constraint
        }
    }

    request = json.loads(
        await ledger.build_auth_rule_request(identifier, txn_type, add_auth_action, field, None, new_value,
                                             json.dumps(constraint)))
    assert expected_request.items() <= request.items()


@pytest.mark.asyncio
async def test_build_auth_rule_request_works_for_edit_auth_action():
    expected_request = {
        "identifier": identifier,
        "operation": {
            "type": "120",
            "auth_type": "1",
            "auth_action": edit_auth_action,
            "field": field,
            "old_value": old_value,
            "new_value": new_value,
            "constraint": constraint
        }
    }

    request = json.loads(
        await ledger.build_auth_rule_request(identifier, txn_type, edit_auth_action, field, old_value, new_value,
                                             json.dumps(constraint)))
    assert expected_request.items() <= request.items()
