from tests.utils import storage
from indy import ledger
from indy.error import ErrorCode, IndyError

import json
import pytest
import logging

logging.basicConfig(level=logging.DEBUG)


@pytest.fixture(autouse=True)
def before_after_each():
    storage.cleanup()
    yield
    storage.cleanup()


@pytest.mark.asyncio
async def test_build_node_request_works_for_missed_field_in_data_json():
    identifier = "identifier"
    destination = "destination"
    data = {
        "node_ip": "ip",
        "node_port": 1,
        "client_ip": "ip",
        "client_port": 1
    }

    try:
        await ledger.build_node_request(identifier, destination, json.dumps(data))
        raise Exception("Failed")
    except Exception as e:
        assert type(IndyError(ErrorCode.CommonInvalidStructure)) == type(e) and \
               IndyError(ErrorCode.CommonInvalidStructure).args == e.args
