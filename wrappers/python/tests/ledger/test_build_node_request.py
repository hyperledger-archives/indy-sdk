from indy import ledger
from indy.error import ErrorCode, IndyError

import json
import pytest


@pytest.mark.asyncio
async def test_build_node_request_works_for_missed_fields_in_data_json(did_trustee):
    destination = "destination"
    data = { }

    with pytest.raises(IndyError) as e:
        await ledger.build_node_request(did_trustee, destination, json.dumps(data))
    assert ErrorCode.CommonInvalidStructure == e.value.error_code


@pytest.mark.asyncio
async def test_build_node_request_works_for_correct_data_json(did_trustee):
    destination = "destination"
    data = {
        "node_ip": "ip",
        "node_port": 1,
        "client_ip": "ip",
        "client_port": 1,
        "alias": "some",
        "services": ["VALIDATOR"],
        "blskey": "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    }

    expected_response = {
        "identifier": did_trustee,
        "operation": {
            "type": "0",
            "dest": destination,
            "data": data
        }
    }

    response = json.loads(await ledger.build_node_request(did_trustee, destination, json.dumps(data)))
    assert expected_response.items() <= response.items()
