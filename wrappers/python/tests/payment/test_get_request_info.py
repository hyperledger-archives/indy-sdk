from indy import payment

import json
import pytest


get_auth_rule_resp = {
    "result": {
        "data": [{
            "new_value": "0",
            "constraint": {
                "need_to_be_owner": False,
                "sig_count": 1,
                "metadata": {
                    "fees": "1"
                },
                "role": "0",
                "constraint_id": "ROLE"
            },
            "field": "role",
            "auth_type": "1",
            "auth_action": "ADD"
        }],
        "identifier": "LibindyDid111111111111",
        "auth_action": "ADD",
        "new_value": "0",
        "reqId": 15616,
        "auth_type": "1",
        "type": "121",
        "field": "role"
    },
    "op": "REPLY"
}
requester_info = {
    "role": "0",
    "need_to_be_owner": False,
    "sig_count": 1,
}
fees = {
    "1": 100
}


@pytest.mark.asyncio
async def test_get_request_info():
    request_info = await payment.get_request_info(json.dumps(get_auth_rule_resp), json.dumps(requester_info),
                                                  json.dumps(fees))
    expected_request_info = {
        'price': 100,
        'requirements': [{
            'role': '0',
            'need_to_be_owner': False,
            'sig_count': 1,
        }]
    }

    request_info = json.loads(request_info)
    assert expected_request_info == request_info
